########################
# Type hints in python #
########################

# Resources:
#  - https://docs.python.org/3/library/typing.html#module-typing
#  - https://typing.readthedocs.io/en/latest
#  - https://mypy.readthedocs.io/en/stable/cheat_sheet_py3.html


# Firstly why type hints?
# - Type hints are a form of self-documentation.
# - Type hints can aid development as IDEs provide better autocompletion
#    and type checking.
# - Type hints can help catch bugs though static analysis.

# Why not type hints?
# - Type hints can be hard to get right and sync time for generic code.
# - Type hints may slow you down for through away code.

import time
import typing
from collections.abc import Callable, Iterable, Sequence, Sized
from functools import wraps
from typing import (
    Any,
    ClassVar,
    Generic,
    Literal,
    NamedTuple,
    NotRequired,
    Protocol,
    Self,
    TypeAlias,
    TypeGuard,
    TypedDict,
    TypeVar,
    overload,
    runtime_checkable,
)

name: str = "James"
item_count: int = 3
height: float = 1.79
is_available: bool = True

# We can also use generics which are types that can be parametrised
# the most common generic types are list, dict, set, tuple
# Prior to python 3.9 you would have to import List, Dict, Set, Tuple
# from the typing module
customers: list[str] = ["Jim", "Jane", "John"]
customers.append(name)

# This shows a more nested example:
customer_names: list[tuple[str, str]] = [
    ("Jim", "ward"),
    ("Jane", "jones"),
    ("John", "darcy"),
]


staff: dict[str, str | list[str] | None] = {
    "Manager": "Bob",
    "Accountants": ["Jane", "John"],
    "Cleaners": None,
}
# Note pre python 3.10 you would have to use the following syntax
#
# staff: Dict[str, Optional[Union[str, List[str]]]]
#
# The | operator is a shorthand for Union introduced in python 3.10
# Optional[T] is a shorthand for Union[T, None] or T | None
#
# Note that list only accepts one type, if you want to accept multiple
# types you can use the typing module's Union type or it
# might be an indication that you should use a tuple instead
# or a custom class.
# E.g.
bob_customer: list[str | int] = ["Bob", 30]
# might be better as
bob_customer_two: tuple[str, int] = ("Bob", 30)
# Note we can still type hint tuples with one fixed type
ages: tuple[int, ...] = (30, 40, 50)


# In this example it might be better to use a named tuple
class Customer1(NamedTuple):
    name: str
    age: int


bob_customer_three: Customer1 = Customer1("Bob", 30)


#########################
# Static type checking  #
#########################

# We can use a static type checker such as mypy
# or pyright to check our type hints for errors.

# If you have a type checker like Pylance in vscode
# uncomment the below to show and error as customers
# is expected to be a list of strings

# customers = "bob, janet"

###############
# type ignore #
###############

# The following line shows how to ignore incorrect type hints
# Remove the comment to see the error highlighting
staff["_employees"] = 3  # type: ignore

#######
# Any #
#######

# We can use Any to indicate that a variable can be any type
# note no error
anything: Any = 3
anything = "hello"

###############
# reveal_type #
###############


# The reveal_type function is a special function that
# can be used to show the type of a variable.
if typing.TYPE_CHECKING:
    # Run mypy on this file to see the reveal_type output
    # mypy python/teaching-python/type_hints_basics.py
    reveal_type(customers)

###########
# Literal #
###########


SwitchStatus = Literal["on", "off"]


class Light:
    def __init__(self, status: SwitchStatus) -> None:
        self.status = status

    def set_switch(self, status: SwitchStatus) -> None:
        self.status = status

    @property
    def is_on(self) -> bool:
        return self.status == "on"

    def display(self) -> str:
        return "💡" if self.is_on else ""


lamp = Light("on")

# Uncomment below to see the error.
# Note it is often good to use an enum instead of a literal
# to make it more readable and maintainable.

# lamp.set_switch("Off")


##############
# Type Alias #
##############

Customers: TypeAlias = list[str]
more_customers: Customers = ["Sandra", "Sally", "Sue"]

##################
# TypedDict Type #
##################

# Rather than just using a dictionary we can use TypedDict
# better than dict[str, str] as it is more descriptive
# and can help to catch errors.


class CustomerData(TypedDict):
    name: str
    surname: str
    age: NotRequired[int]


bill: CustomerData = {"name": "Bill", "surname": "Smith"}


# Uncomment the following lines to see the error highlighting.
# The three examples show, typos, missing fields and extra fields.

# bad_bill_1:Customer = {"name": "Bill", "sur_name": "Smith"}
# bad_bill_2:Customer = {"name": "Bill"}
# bad_bill_2:Customer = {**bill, "height": 177}

# Also try bill[] in the editor to see the available fields.


######################
# User Defined Types #
######################


class Customer:
    legacy: ClassVar[bool] = False

    def __init__(self, name: str, surname: str, age: int = 0) -> None:
        self.name = name
        self.surname = surname
        self.age = age

    @classmethod
    def from_customer_data(cls, data: CustomerData) -> "Customer":
        return Customer(data["name"], data["surname"])

    def reset_age(self) -> Self:
        self.age = 0
        return self


custom_customer: Customer = Customer("Bob", "Geldof")

# Note to type hint the class itself we can use type[ClassName]
# Prior to python 3.9 you would use typing.Type[ClassName]

CustomerClass: type[Customer] = Customer


#######################
# Function Signatures #
#######################

# In the function below we show how to define
# function argument types and return types.


def add_surname(name: str) -> str:
    return name + " Smith"


# In the function below we show how to define
# types for *args and **kwargs.
# Note all args are considered to be of type int
# and all kwargs are considered to be of type str.

# Hence args: tuple[int, ...] and kwargs: dict[str, str]


def args_and_kwargs(*args: int, **kwargs: str) -> None:
    print(args)
    print(kwargs)


##################
# Type narrowing #
##################


# Type narrowing is a feature of a type checker that allows
# the type checker to infer a more specific type for a variable
# e.g. hover over the employee variable to see the type
# in each branch.


def display_staff(staff: dict[str, str | list[str] | None]) -> None:
    for role, employees in staff.items():
        if employees is None:
            print(f"No {role}")
        elif isinstance(employees, str):
            print(f"{role}: {employees}")
        else:
            print(f"{role}: {', '.join(employees)}")


##################
# Callable types #
##################


def modify_customers(
    customers: Iterable[str], modifier: Callable[[str], str]
) -> list[str]:
    print(f"Modifying customers with {modifier.__name__}")
    return [modifier(customer) for customer in customers]


customers_with_surnames = modify_customers(customers, add_surname)

# Notice the use of Iterable and Callable from the typing module.

# Tip:
# In general use abstract types for input arguments and concrete types
# for output arguments. Iterable is an abstract type that can be used
# for any iterable type such as lists, tuples, sets.
# For example:

tuple_customers = modify_customers(("Jim", "Jane", "John"), add_surname)


###################################
# Structural typing and Protocols #
###################################

# Prior to # https://peps.python.org/pep-0544/
# The only way to specify typing was via nominal typing
# i.e. you would have to inherit from a class
# to be considered a certain type.

# With the introduction of protocols we can now use structural
# typing i.e. we can specify that a class has certain attributes
# without having to inherit from a class.

# For example we can define a protocol for a class that
# has a display method.


class HasDisplay(Protocol):
    def display(self) -> None: ...


class Human:
    planet: ClassVar[str] = "Earth"

    def __init__(self, name: str, age: int) -> None:
        self._name = name
        self.age = age

    def display(self) -> None:
        print(f"{self._name} is {self.age} years old")


def display(entity: HasDisplay) -> None:
    print("Displaying entity:")
    entity.display()


age = 10
bob: Human | None
if age < 100:
    bob = Human("Bob", age)
    display(bob)
else:
    bob = None

# Note the standard library has a lot of protocols defined
# e.g.
# - Sized
# - Iterable
# - Container
# - Hashable
# - Reversible
# - SupportsAbs
# - SupportsBytes
# - SupportsComplex
# - SupportsFloat
# - SupportsInt
# - SupportsRound

# Note at runtime bob is not considered to be of type HasDisplay.

# In fact running isinstance(bob, HasDisplay) will raise an error.
# If you want to enable runtime checking you can use the
# runtime_checkable decorator.


@runtime_checkable
class HasDisplay2(Protocol):
    def display(self) -> None: ...


print("Is bob an instance of HasDisplay2?", isinstance(bob, HasDisplay2))


# Note the use of the ClassVar attribute in the Human class


##################
# Type Variables #
##################


# We can also parametrise types for more flexibility

T = TypeVar("T")


def first_item(items: Sequence[T]) -> T:
    return items[0]


first_customer = first_item(customers)

# Note hovering over first_customer will show the type as str.
# Since python 3.12 we can use the following syntax


def first_item_new[T](items: Sequence[T]) -> T:
    return items[0]


list_of_sized: list[Sized] = [customers, customer_names, "simon"]

S = TypeVar("S", bound=Sized)


def zip_with_size(items: Iterable[S]) -> list[tuple[S, int]]:
    return [(item, len(item)) for item in items]


def zip_with_size2[T: (Sized)](items: Iterable[T]) -> list[tuple[T, int]]:
    return [(item, len(item)) for item in items]


##########################################
# ParamSpecs, Concat and Type Var tuples #
##########################################

# https://peps.python.org/pep-0612/

# imagine we want to decorate a function with a slow down decorator


def slow_down(func: Callable[..., Any]) -> Callable[..., Any]:
    def wrapper(*args: Any, **kwargs: Any) -> Any:
        time.sleep(1)
        result = func(*args, **kwargs)
        return result

    return wrapper


slow_add_surname = slow_down(add_surname)
# slow_add_surname has the type Callable[..., str] but we know
# it should have the type Callable[[str], str]

# We can use param specs concat to fix this
# where P represents the parameter types of the function
# we are decorating.


def slow_down2[T, **P](func: Callable[P, T]) -> Callable[P, T]:
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
        time.sleep(1)
        result = func(*args, **kwargs)
        return result

    return wrapper


slow_add_surname_2 = slow_down2(add_surname)


def complex_function(
    a: int, b: str, *args: str, flag: bool = True, **kwargs: Any
) -> str | None:
    if flag:
        return None
    return "".join((a * b, *args))


slow_complex_function = slow_down2(complex_function)


#########################
# User Defined generics #
#########################


class Pair[T1, T2]:
    def __init__(self, first: T1, second: T2) -> None:
        self.first: T1 = first
        self.second: T2 = second

    def swap(self) -> "Pair[T2, T1]":
        return Pair(self.second, self.first)


def swap_pair[T1, T2](pair: Pair[T1, T2]) -> Pair[T2, T1]:
    return Pair(pair.second, pair.first)


pair = Pair(1, "one")
swapped_pair = swap_pair(pair)

# before 2.12 we would user the Generic class and use type vars

T1 = TypeVar("T1")
T2 = TypeVar("T2")


class Pair2(Generic[T1, T2]):
    def __init__(self, first: T1, second: T2) -> None:
        self.first: T1 = first
        self.second: T2 = second


# Note that the type checker will infer the types of the swapped_pair
# variable as Pair[str, int].


###############
# Overloading #
###############


@overload
def skip_first[A, *Ts](items: tuple[A, *Ts]) -> tuple[*Ts]: ...


@overload
def skip_first(items: tuple[()]) -> tuple[()]: ...


def skip_first[A, *Ts](items: tuple[A, *Ts] | tuple[()]) -> tuple[*Ts] | tuple[()]:
    return items[1:]


empty_tuple = skip_first(())
string_float_pair = skip_first((1, "2", 3.0))

# In this case both types are correctly inferred. Note this
# also makes use of type var tuples introduced in python 3.10
# and the new tuple syntax introduced in python 3.12

####################################################
# Type hinting a decorator with optional arguments #
####################################################

# Let's say we want a configurable sleep decorator


@overload
def sleep_decorator2[
    **P, T
](func: Callable[P, T], *, seconds: int = 1) -> Callable[P, T]: ...


@overload
def sleep_decorator2[
    **P, T
](func: None, *, seconds: int = 1) -> Callable[[Callable[P, T]], Callable[P, T]]: ...


def sleep_decorator2[
    **P, T
](func: Callable[P, T] | None = None, *, seconds: int = 1) -> (
    Callable[P, T] | Callable[[Callable[P, T]], Callable[P, T]]
):
    def decorator(func: Callable[P, T]) -> Callable[P, T]:
        @wraps(func)
        def wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
            time.sleep(seconds)
            result = func(*args, **kwargs)
            return result

        return wrapper

    if func is None:
        return decorator
    return decorator(func)


@sleep_decorator2
def or_hi(a: str, b: bool = False):
    return a if b else "hi"


if typing.TYPE_CHECKING:
    reveal_type(or_hi)


##############
# Type Guard #
##############

# https://peps.python.org/pep-0647/


def has_multiple_employees(employees: str | list[str] | None) -> bool:
    return isinstance(employees, list) and all(
        not employee.endswith("bot") for employee in employees
    )


def get_first_department(
    staff: dict[str, str | list[str] | None]
) -> str | list[str] | None:
    for employees in staff.values():
        if has_multiple_employees(employees):
            return employees
    return None


department = get_first_department(staff)

# In the above code the type checker will infer the type of the
# department variable as str | list[str] | None

# However we know that the department variable will always be a list
# of strings or None. We can use a type guard to narrow the type!

# Type guards are used to tell the type checker that the return type
# is a bool and that the input type is a list of strings when the
# function returns True.


def has_multiple_employees2(employees: str | list[str] | None) -> TypeGuard[list[str]]:
    return isinstance(employees, list) and all(
        not employee.endswith("bot") for employee in employees
    )


def get_first_department2(staff: dict[str, str | list[str] | None]) -> list[str] | None:
    for employees in staff.values():
        if has_multiple_employees2(employees):
            return employees
    return None


department2 = get_first_department2(staff)
