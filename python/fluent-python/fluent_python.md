# Fluent Python


Recently whilst reading `Fluent Python` by Luciano Ramalho I decided to write this post with a collection of my favourite snippets and Python features as I read the book. These will be broadly based on the contents of the book however with my own twist. 

### Python data model - bool and `__bool__`


The python data model allows you to implement special methods that make custom types behave like the builtin types. These dunder methods allow for idomatic python or so called pythonic code.



```python
class Example:
    def __len__(self):
        print(f"running __len__")
        return 10

    def __repr__(self):
        print(f"running __repr__")
        return f"Example()"

    def __str__(self):
        print(f"running __str__")
        return f"<Example Python object"

    def __bool__(self):
        print(f"running __bool__")
        return True

```


```python
example = Example()
example

```

    running __repr__





    Example()




```python
len(example)

```

    running __len__





    10




```python
if example:
    pass

```

    running __bool__


To check the whether `example` is truthy it calls effectively calls `bool(example)` which looks for `example.__bool__()`
If there is no `__bool__` python tries to invoke `__len__` and if that returns 0 then `False` else `True`.


### `__repr__` and `__str__`


`__repr__` - unambiguous for developers.  
`__str__` - for the end user.


### Unpacking sequences



```python
person_data = ("Simon", "Ward-Jones", 1, 2, 3)
name, surname, *numbers = person_data
print(name, surname, numbers, sep="\n")

```

    Simon
    Ward-Jones
    [1, 2, 3]



```python
# Unpacking into a list
person_data = [*person_data]
person_data

```




    ['Simon', 'Ward-Jones', 1, 2, 3]



### Bad matrix



```python
matrix = [[None] * 3] * 3
matrix[0][0] = 1
matrix

```




    [[1, None, None], [1, None, None], [1, None, None]]




```python
# you probably want:
matrix = []
for _ in range(3):
    matrix.append([None] * 3)
matrix[0][0] = 1
matrix

```




    [[1, None, None], [None, None, None], [None, None, None]]



### Key in min, max and sorted



```python
numbers = [3, "5", 1, "10", 2, "4", "6"]

```


```python
print(sorted(numbers, key=int))
print(min(numbers, key=int))

```

    [1, 2, 3, '4', '5', '6', '10']
    1


### Unpacking mappings



```python
person_data_1 = {"name": "Simon", "surname": "Ward-Jones", "age": 20}
person_data_2 = {
    "name": "Simon",
    "surname": "Ward-Jones",
    "age": 30,
    "height": 2,
    "weight": 3,
}

```


```python
{**person_data_1, **person_data_2}

```




    {'name': 'Simon', 'surname': 'Ward-Jones', 'age': 30, 'height': 2, 'weight': 3}




```python
person_data_1 | person_data_2

```




    {'name': 'Simon', 'surname': 'Ward-Jones', 'age': 30, 'height': 2, 'weight': 3}



### Dictionary comprehensions



```python
country_population_pairs = [
    ("China", 1409517397),
    ("India", 1324171354),
    ("United States", 325234000),
    ("Indonesia", 255461563),
    ("Brazil", 210147125),
    ("Pakistan", 197015955),
    ("Nigeria", 206139589),
    ("Bangladesh", 164895000),
    ("Russia", 146585181),
    ("Japan", 126470000),
]

```


```python
# create a dictionary of country populations
country_populations = {
    country: population for country, population in country_population_pairs
}

```

### Dictionary setdefault method



```python
person_data_1 = {"name": "Simon", "surname": "Ward-Jones", "age": 20}
person_data_2 = {
    "name": "Simon",
    "surname": "Ward-Jones",
    "age": 30,
    "height": 2,
    "weight": 3,
}

```


```python
# set default looks up the key and returns the value if it exists
# if it doesn't exist, it sets the key to the default value and returns the default value
person_data_1.setdefault("name", "John")

```




    'Simon'




```python
# This is really useful for something like this:
person_data_1.setdefault("animals", {}).setdefault("cats", []).append("cat")
person_data_1

```




    {'name': 'Simon',
     'surname': 'Ward-Jones',
     'age': 20,
     'animals': {'cats': ['cat']}}



### Dictionary views and mappings work like sets



```python
person_data_1 = {"name": "Simon", "surname": "Ward-Jones", "age": 20}
person_data_2 = {
    "name": "Simon",
    "surname": "Ward-Jones",
    "age": 30,
    "height": 2,
    "weight": 3,
}
# finding common keys
person_data_1.keys() & person_data_2.keys()

```




    {'age', 'name', 'surname'}



### Unicode names



```python
from unicodedata import name
import random

```


```python
n = 0
while n < 5:
    unicode_point = chr(random.randint(0, 0x10FFFF))  # pick a random unicode point
    if unicode_name := name(unicode_point, ""):
        print(unicode_point, unicode_name)  # print the point and the name
        n += 1

```

    𧶖 CJK UNIFIED IDEOGRAPH-27D96
    𨄼 CJK UNIFIED IDEOGRAPH-2813C
    ꄩ YI SYLLABLE TOT
    Ḋ LATIN CAPITAL LETTER D WITH DOT ABOVE
    𣚴 CJK UNIFIED IDEOGRAPH-236B4


### string translation



```python
translation = str.maketrans("simon", "abcde")

```


```python
"hello simon".translate(translation)

```




    'helld abcde'



### Dataclasses save you a lot of typing



```python
from dataclasses import dataclass
from dataclasses import field
from typing import Optional


@dataclass
class Book:
    title: str
    author: Optional[str] = "unknown"
    pages: int = 0
    price: float = field(default=0.0, repr=False)

    def __post_init__(self):
        self.long_title = self.title + " by " + self.author

```


```python
book = Book(title="Python Programming", author="Simon Ward-Jones", pages=300)
book

```




    Book(title='Python Programming', author='Simon Ward-Jones', pages=300)



### Variables are not boxes 📦 🏷



```python
variable = ["📦 ", "🏷"]
other_variable = variable
other_variable.append("📦")
variable  # say what...

```




    ['📦 ', '🏷', '📦']



### Copy is only one level deep



```python
data = [1, 2, [3, 4]]
new_data = data.copy()
for item, new_item in zip(data, new_data):
    print(id(item), id(new_item))

```

    4554309872 4554309872
    4554309904 4554309904
    4604433856 4604433856


### Watch out for passing a mutable object to a function



```python
# the function will modify the mutable data passed to it
data = [1, 2, 3, 4]


def sum_all_but_last(data):
    last_item = data.pop()
    print(f"Removing last item before sum: {last_item}")
    return sum(data)


data_sum_all_but_last = sum_all_but_last(data)
print(f"The sum is {data_sum_all_but_last} but data is now: {data}")

```

    Removing last item before sum: 4
    The sum is 6 but data is now: [1, 2, 3]


### Really watch out for mutable default arguments



```python
class Person:
    def __init__(self, name, pets=[]):
        self.name = name
        self.pets = pets


simon = Person("Simon")
paul = Person("Paul")
paul.pets.append("cat")
# Now simon has a pet cat too!
simon.pets

```




    ['cat']



### Classic late binding lambdas trick



```python
# The value of i is looked up at execution time.
for function in [lambda x: x + i for i in range(3)]:
    print(function(1))

```

    3
    3
    3


### Tuples are immutable but their contents can be changed



```python
address = "42 Wallaby Way, Sydney"
rooms = ["kitchen", "living room", "dining room", "bathroom"]
house = (address, rooms)

```


```python
house[1].append("office")
house

```




    ('42 Wallaby Way, Sydney',
     ['kitchen', 'living room', 'dining room', 'bathroom', 'office'])



### The operator module is handy when reducing



```python
from operator import mul
from functools import reduce

numbers = [3, 4, 5, 6]
product_of_numbers = reduce(mul, numbers)

# without the mul operator:
# product_of_numbers = reduce(lambda x, y: x * y, numbers)

print(product_of_numbers)

```

    360


### Use the operator itemgetter to sort a list by specific item



```python
from operator import itemgetter

sorted([("a", 2), ("b", 1), ("c", 3)], key=itemgetter(1))

# This is the same as doing:
# sorted([("a", 2), ("b", 1), ("c", 3)], key=lambda x: x[1])

```




    [('b', 1), ('a', 2), ('c', 3)]



### Use the operator attrgetter to sort by specific (maybe nested) attribute



```python
from operator import attrgetter
from typing import NamedTuple, Optional


class Pet(NamedTuple):
    name: str
    species: str
    age: int


class Human(NamedTuple):
    name: str
    pet: Pet


people = [
    Human(name="Simon", pet=Pet(name="Whiskers", species="cat", age=3)),
    Human(name="Richard", pet=Pet(name="Giles", species="shark", age=1)),
    Human(name="Paul", pet=Pet(name="Alfie", species="dog", age=2)),
    Human(name="Tom", pet=Pet(name="Rupert", species="cat", age=3)),
]

```


```python
# sort by animal age ascending
sorted(people, key=attrgetter("pet.age"))

# This is equivalent to:
# sorted(people, key=lambda x: x.pet.age)

```




    [Human(name='Richard', pet=Pet(name='Giles', species='shark', age=1)),
     Human(name='Paul', pet=Pet(name='Alfie', species='dog', age=2)),
     Human(name='Simon', pet=Pet(name='Whiskers', species='cat', age=3)),
     Human(name='Tom', pet=Pet(name='Rupert', species='cat', age=3))]




```python
# sort by animal age then by owner name (note Simon's 3 year olf pet comes before Tom's)
sorted(people, key=attrgetter("pet.age", "name"))

# This is equivalent to:
# sorted(people, key=lambda x: (x.pet.age, x.name))

```




    [Human(name='Richard', pet=Pet(name='Giles', species='shark', age=1)),
     Human(name='Paul', pet=Pet(name='Alfie', species='dog', age=2)),
     Human(name='Simon', pet=Pet(name='Whiskers', species='cat', age=3)),
     Human(name='Tom', pet=Pet(name='Rupert', species='cat', age=3))]



### Key word only arguments



```python
def count_letters(string: str, *, letters: str = "aeiou") -> int:
    return sum(1 for c in string if c.lower() in letters)


# This call would fail:
# count_letters("simon", "aei")
# One must call with letters key word like:
count_letters("simon", letters="aei")

```




    1



### Positional only parameters



```python
def count_letters(string: str, /, letters: str = "aeiou") -> int:
    return sum(1 for c in string if c.lower() in letters)


# This call would fail:
# count_letters(string="simon", letters="aei")
# One must call using positional argument like:
count_letters("simon", "aei")

```




    1



### Positional only parameters and Keyword only parameters



```python
# Note we can combine positional only with keyword only arguments


def count_letters(
    string: str, /, letters: str = "aeiou", *, case_sensitive: bool = False
) -> int:
    if not case_sensitive:
        string = string.lower()
    return sum(1 for c in string if c.lower() in letters)


# In this case:
#     string must be a positional argument
#     letters can be a keyword argument or a positional argument
#     case_sensitive mst be a keyword argument

count_letters("simon", letters="aei", case_sensitive=True)
count_letters("simon", "aei", case_sensitive=True)

```




    1



### Closures capture variable values in the outer scope



```python
def get_wife_function():
    wives = ()

    def add_wife(name: str):
        nonlocal wives
        wives += (name,)
        return wives

    return add_wife


henrys_wives = get_wife_function()

henrys_wives("catherine_of_aragon")
henrys_wives("anne_boleyn")
henrys_wives("jane_seymour")
henrys_wives("anne_of_cleves")
henrys_wives("catherine_howard")
wives = henrys_wives("catherine_parr")


print(henrys_wives.__code__.co_varnames)
print(henrys_wives.__code__.co_freevars)

print(wives)
print(henrys_wives.__closure__[0].cell_contents)

```

    ('name',)
    ('wives',)
    ('catherine_of_aragon', 'anne_boleyn', 'jane_seymour', 'anne_of_cleves', 'catherine_howard', 'catherine_parr')
    ('catherine_of_aragon', 'anne_boleyn', 'jane_seymour', 'anne_of_cleves', 'catherine_howard', 'catherine_parr')


### Another example of scope to create a simple counter



```python
# We specify non local to reference a variable that is in the outer scope.
# If we did not we would get an error.


def get_counter():
    count = 0

    def counter():
        nonlocal count
        count += 1
        return count

    return counter


counter = get_counter()

while (x := counter()) < 5:
    print(x)

```

    1
    2
    3
    4


### Use single dispatch to implement polymorphism



```python
# The function called depends on the type at runtime.

from typing import NamedTuple
from collections.abc import Iterable
from functools import singledispatch
import re


class RGB(NamedTuple):
    red: int
    green: int
    blue: int


@singledispatch
def get_hex(rgb: Iterable[int]) -> str:
    print("Using str implementation")
    red, green, blue = rgb
    return f"#{red:02x}{green:02x}{blue:02x}"


@get_hex.register
def _(rgb: RGB) -> str:
    print("Using RGB implementation")
    return f"#{rgb.red:02x}{rgb.green:02x}{rgb.blue:02x}"


@get_hex.register
def _(rgb: str) -> str:
    red, green, blue = (int(x) for x in re.findall(r"\d+", rgb))
    print("Using generic Iterator implementation")
    return f"#{red:02x}{green:02x}{blue:02x}"


print(get_hex("RGB(10, 20, 30)"))
print(get_hex(RGB(red=10, green=20, blue=30)))
print(get_hex((10, 20, 30)))

```

    Using generic Iterator implementation
    #0a141e
    Using RGB implementation
    #0a141e
    Using str implementation
    #0a141e


### Static Duck Typing with Protocols and runtime_checks.



```python
from typing import Protocol, runtime_checkable


@runtime_checkable
class Dog(Protocol):
    def bark(self) -> str:
        ...


class Dalmatian:
    def bark(self) -> str:
        return "woof woof"


# These only work as we used the runtime_checkable decorator.
assert isinstance(Dalmatian(), Dog)
assert issubclass(Dalmatian, Dog)

```

### Registering a virtual subclass



```python
import abc


class Duck(abc.ABC):
    @abc.abstractmethod
    def quack(self):
        pass


@Duck.register
class Goose:
    def quack(self) -> None:
        print("Quack!")


goose = Goose()

assert isinstance(goose, Duck)
assert issubclass(Goose, Duck)

```

### Using the `__subclasshook__`



```python
import abc


class Duck(abc.ABC):
    @classmethod
    def __subclasshook__(cls, instance) -> None:
        if any("quack" in B.__dict__ for B in instance.__mro__):
            return True
        return NotImplemented

    @abc.abstractmethod
    def quack(self):
        pass


class Goose:
    def quack(self) -> None:
        print("Quack!")


goose = Goose()

assert isinstance(goose, Duck)
assert issubclass(Goose, Duck)

```

### Nested f-strings are a thing!



```python
import math

for points in range(5):
    print(f"{math.e:.{points}f}")

```

    3
    2.7
    2.72
    2.718
    2.7183


### f-string tips



```python
format(math.pi, ".4f")

```




    '3.1416'



### Formatting spec mini language



```python
# https://docs.python.org/3/library/string.html#format-specification-mini-language

examples = [
    (9992, "->10"),
    (9992, "*<10"),
    (9992, "^10"),
    (9992, "^10_"),
    (9992, "^10,"),
    (9992, "^+10"),
    (9992, "^+10.0%"),
    (9992, "=+10.2f"),
    (9992, "10.2G"),
    (9992, "b"),
    (9992, "^10c"),
    (9992, "^10d"),
    (9992, "^10x"),
    (9992, "^#10x"),
    (1234.56789, "^10,.2f"),
    (1234.56789, "^10.2G"),
]
cw = 15
print(f"|{'Value':^12}|{'Spec':^10}|{'Output':^20}|")
for value, format_spec in examples:
    output = f"'{value:{format_spec}}'"
    print(f"|{value:^12}|{format_spec:^10}|{output:^20}|")

```

    |   Value    |   Spec   |       Output       |
    |    9992    |   ->10   |    '------9992'    |
    |    9992    |   *<10   |    '9992******'    |
    |    9992    |   ^10    |    '   9992   '    |
    |    9992    |   ^10_   |    '  9_992   '    |
    |    9992    |   ^10,   |    '  9,992   '    |
    |    9992    |   ^+10   |    '  +9992   '    |
    |    9992    | ^+10.0%  |    ' +999200% '    |
    |    9992    | =+10.2f  |    '+  9992.00'    |
    |    9992    |  10.2G   |    '     1E+04'    |
    |    9992    |    b     |  '10011100001000'  |
    |    9992    |   ^10c   |    '    ✈     '    |
    |    9992    |   ^10d   |    '   9992   '    |
    |    9992    |   ^10x   |    '   2708   '    |
    |    9992    |  ^#10x   |    '  0x2708  '    |
    | 1234.56789 | ^10,.2f  |    ' 1,234.57 '    |
    | 1234.56789 |  ^10.2G  |    ' 1.2E+03  '    |


### conversion flags in f strings



```python
import math

emoji = "✈️"

print(f"!a calls ascii(): {emoji!a}")
print(f"!s calls str(): {emoji!s}")
print(f"!r calls repr(): {emoji!r}")

```

    !a calls ascii(): '\u2708\ufe0f'
    !s calls str(): ✈️
    !r calls repr(): '✈️'


### Enhance f-string with `__format__`



```python
import re


class Plane:
    def __repr__(self):
        return f"{self.__class__.__name__}()"

    def __str__(self):
        return f"✈️"

    def __format__(self, __format_spec: str):
        if match := re.match(r"(.*?)(\d+)?r", __format_spec):
            repeat = int(match[2]) if match[2] else 2
            return super().__format__(match[1]) * repeat
        return super().__format__(__format_spec)


plane = Plane()

print(f"By default f-strings use __str__ = {plane}")
print(f"Force use of __repr__ with !r = {plane!r}")
print(f"Demo of custom format spec = {plane:r}")
print(f"Demo of custom format spec = {plane:2r}")
print(f"Demo of custom format spec = {plane:5r}")

```

    By default f-strings use __str__ = ✈️
    Force use of __repr__ with !r = Plane()
    Demo of custom format spec = ✈️✈️
    Demo of custom format spec = ✈️✈️
    Demo of custom format spec = ✈️✈️✈️✈️✈️


### Customising the `__match_args__`



```python
class Cow:
    __match_args__ = ("age", "breed", "info")

    def __init__(self, age: int, breed: str, info: dict[str:int]) -> None:
        self.age = age
        self.breed = breed
        self.info = info


cow = Cow(10, "Jersey", {"weight": 600})

match cow:
    case Cow(10 as age, name, {"weight": weight} as info):
        print(age, name, weight, info)

```

    10 Jersey 600 {'weight': 600}


### Itertools filtering generator functions



```python
import itertools

print(list(filter(str.isupper, "HiEhdsLsaLsasaO")))
print(list(itertools.filterfalse(str.isupper, "HiEhdsLsaLsasaO")))
print(list(itertools.dropwhile(str.isupper, "HiEhdsLsaLsasaO")))
print(list(itertools.takewhile(str.isupper, "HiEhdsLsaLsasaO")))

filter_by = (0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1)
print(list(itertools.compress("HiEhdsLsaLsasaO", filter_by)))

```

    ['H', 'E', 'L', 'L', 'O']
    ['i', 'h', 'd', 's', 's', 'a', 's', 'a', 's', 'a']
    ['i', 'E', 'h', 'd', 's', 'L', 's', 'a', 'L', 's', 'a', 's', 'a', 'O']
    ['H']
    ['i', 's', 'L', 'a', 's', 's', 'O']


### Itertools mapping generator functions



```python
import itertools
from operator import mul

numbers = [2, 6, 3, 78, 4, 9, 6, 0, 3, 100]

print(list(itertools.accumulate(numbers)))
print(list(itertools.accumulate(numbers, min)))
print(list(itertools.accumulate(numbers, max)))
print(list(itertools.accumulate(numbers, mul)))
print(list(enumerate(numbers)))
print(list(map(lambda x: x**2, numbers)))
print(list(itertools.starmap(lambda x, y: x + 2 * y, enumerate(numbers))))

```

    [2, 8, 11, 89, 93, 102, 108, 108, 111, 211]
    [2, 2, 2, 2, 2, 2, 2, 0, 0, 0]
    [2, 6, 6, 78, 78, 78, 78, 78, 78, 100]
    [2, 12, 36, 2808, 11232, 101088, 606528, 0, 0, 0]
    [(0, 2), (1, 6), (2, 3), (3, 78), (4, 4), (5, 9), (6, 6), (7, 0), (8, 3), (9, 100)]
    [4, 36, 9, 6084, 16, 81, 36, 0, 9, 10000]
    [4, 13, 8, 159, 12, 23, 18, 7, 14, 209]


### Itertools merging generators



```python
import itertools

print(list(itertools.chain([1, 2], [3, 4, 5], [6, 7])))
print(list(itertools.chain.from_iterable([[1, 2], [3, 4, 5], [6, 7]])))
print(list(itertools.product([1, 2], [3, 4, 5], [6, 7])))
print(list(zip([1, 2], [3, 4, 5], [6, 7])))
print(list(itertools.zip_longest([1, 2], [3, 4, 5], [6, 7])))

```

    [1, 2, 3, 4, 5, 6, 7]
    [1, 2, 3, 4, 5, 6, 7]
    [(1, 3, 6), (1, 3, 7), (1, 4, 6), (1, 4, 7), (1, 5, 6), (1, 5, 7), (2, 3, 6), (2, 3, 7), (2, 4, 6), (2, 4, 7), (2, 5, 6), (2, 5, 7)]
    [(1, 3, 6), (2, 4, 7)]
    [(1, 3, 6), (2, 4, 7), (None, 5, None)]


### Itertools expanding iterables



```python
import itertools

numbers = [3, 4, 5, 5]

print(list(itertools.combinations(numbers, r=2)))
print(list(itertools.combinations_with_replacement(numbers, r=2)))
print(list(itertools.permutations(numbers, r=2)))
print(list(itertools.product(numbers, repeat=2)))

print(list(itertools.takewhile(lambda x: x < 100, itertools.count(1, 10))))
print(list(itertools.islice(itertools.cycle([1, 10]), 8)))
print(list(itertools.pairwise(range(7))))
print(list(itertools.repeat([1, 10], times=3)))

```

    [(3, 4), (3, 5), (3, 5), (4, 5), (4, 5), (5, 5)]
    [(3, 3), (3, 4), (3, 5), (3, 5), (4, 4), (4, 5), (4, 5), (5, 5), (5, 5), (5, 5)]
    [(3, 4), (3, 5), (3, 5), (4, 3), (4, 5), (4, 5), (5, 3), (5, 4), (5, 5), (5, 3), (5, 4), (5, 5)]
    [(3, 3), (3, 4), (3, 5), (3, 5), (4, 3), (4, 4), (4, 5), (4, 5), (5, 3), (5, 4), (5, 5), (5, 5), (5, 3), (5, 4), (5, 5), (5, 5)]
    [1, 11, 21, 31, 41, 51, 61, 71, 81, 91]
    [1, 10, 1, 10, 1, 10, 1, 10]
    [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]
    [[1, 10], [1, 10], [1, 10]]


### Itertools rearranging generators



```python
import itertools

print([(x, list(y)) for x, y in itertools.groupby("mississippi")])
print(list(reversed("mississippi")))
print(list(zip(*itertools.tee(range(6)))))

```

    [('m', ['m']), ('i', ['i']), ('s', ['s', 's']), ('i', ['i']), ('s', ['s', 's']), ('i', ['i']), ('p', ['p', 'p']), ('i', ['i'])]
    ['i', 'p', 'p', 'i', 's', 's', 'i', 's', 's', 'i', 'm']
    [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]


### yield from



```python
def small_range():
    yield 1
    yield 2
    yield 3


def demo():
    yield from small_range()
    yield 4


for x in demo():
    print(x)

```

    1
    2
    3
    4


### Contextlib can make making a context manager easier



```python
import contextlib
import math


class MathEnv:
    def __enter__(self):
        global e, pi
        pi = math.pi
        e = math.e
        # you can imagine adding more
        return True

    def __exit__(self, exc_type, exc_value, traceback):
        global e, pi
        del pi
        del e


with MathEnv() as this:
    print(this)
    print(e)
    print(pi)

# or with context lib


@contextlib.contextmanager
def math_env():
    global e, pi
    pi = math.pi
    e = math.e
    yield True
    del pi
    del e


with math_env() as this:
    print(this)
    print(e)
    print(pi)

# Also we can do


@math_env()
def using_the_context():
    print(e * pi)


using_the_context()

# This would raise error.
# print(e)

```

    True
    2.718281828459045
    3.141592653589793
    True
    2.718281828459045
    3.141592653589793
    8.539734222673566


### Progress bouncer with threading



```python
from itertools import chain, cycle
from threading import Event, Thread
from time import sleep
import asyncio


def slow_io_bound_function():
    sleep(4)
    return 100


def bouncer(done: Event):
    n = 50
    print("\x1b[?25l", end="")  # hidden
    for i in cycle(chain(range(n), reversed(range(n)))):
        line = "\r┃" + " " * i + "━" + " " * (n - i) + "┃"
        print(line, end="")
        if done.wait(0.05):
            break
    print("\r" + " " * len(line), end="")
    print("\r\x1b[?25h", end="")  # show cursor.


def main():
    done = Event()
    bouncer_task = Thread(target=bouncer, args=(done,))
    bouncer_task.start()
    result = slow_io_bound_function()
    done.set()
    bouncer_task.join()
    return result


main()

```

    [?25h                                                




    100



### Progress bouncer with native coroutines



```python
from itertools import chain, cycle
from threading import Event, Thread
from time import sleep
import asyncio


async def slow_io_bound_function():
    await asyncio.sleep(10)
    return 100


async def bouncer():
    n = 50
    print("\x1b[?25l", end="")  # hidden
    for i in cycle(chain(range(n), reversed(range(n)))):
        line = "\r┃" + " " * i + "━" + " " * (n - i) + "┃"
        print(line, end="", flush=True)
        try:
            await asyncio.sleep(0.05)
        except asyncio.CancelledError:
            break
    print("\r" + " " * len(line), end="")
    print("\r\x1b[?25h", end="")  # show cursor.


async def main():
    bouncer_task = asyncio.create_task(bouncer())
    result = await slow_io_bound_function()
    bouncer_task.cancel()
    return result


# asyncio.run(main()) # this doesn't work in jupyter.
loop = asyncio.get_event_loop()
asyncio.run_coroutine_threadsafe(main(), loop)

None

```

    [?25h                                                

### Customise the fallback `__getattr__` method



```python
from functools import partial
from typing import Any
from collections import abc


class DotMonster:
    def __getattr__(self, value: str):
        if value.lower().startswith("set_"):
            return partial(self.set_value_return_self, value[4:].lower())
        if value.lower().startswith("add_"):
            return partial(self.add_value_return_self, value[4:].lower())
        raise AttributeError(f"{value} is not a valid attribute")

    def set_value_return_self(self, name: str, value: Any):
        setattr(self, name, value)
        return self

    def add_value_return_self(self, name: str, value: Any):
        if not hasattr(self, name):
            if isinstance(value, list):
                setattr(self, name, list(value))
            else:
                setattr(self, name, value)
        elif isinstance(getattr(self, name), list):
            getattr(self, name).append(value)
        else:
            setattr(self, name, [getattr(self, name), value])
        return self

    def __repr__(self):
        return f"{self.__class__.__name__}({self.__dict__})"


monster = DotMonster()

monster.set_name("simon").set_age(105).add_friends("Tom").add_friends("Tim")

# monster.friends
monster

```




    DotMonster({'name': 'simon', 'age': 105, 'friends': ['Tom', 'Tim']})



### MetaClass



```python
class Meta(type):
    def __new__(mcl, cls, bases, namespace):
        args = (mcl, cls, bases, namespace)
        print("Meta.__new__:")
        print("\t", args)
        cls = super().__new__(*args)
        return cls

    @classmethod
    def __prepare__(mcs, class_name, bases):
        print("Meta.__prepare__:")
        print(f"\t{(mcs, class_name, bases)}")
        return {}


class Example(metaclass=Meta):
    def __init__(self):
        print(f"Example.__init__:")
        print(f"\t({self})")

    def __init_subclass__(cls, **kwargs):
        print(f"Example.__init_subclass__:")
        print(f"\t({cls}, {kwargs})")
        return cls


class SubExample(Example):
    a = 10


sub_example = SubExample()

```

    Meta.__prepare__:
    	(<class '__main__.Meta'>, 'Example', ())
    Meta.__new__:
    	 (<class '__main__.Meta'>, 'Example', (), {'__module__': '__main__', '__qualname__': 'Example', '__init__': <function Example.__init__ at 0x112be81f0>, '__init_subclass__': <function Example.__init_subclass__ at 0x112be8280>})
    Meta.__prepare__:
    	(<class '__main__.Meta'>, 'SubExample', (<class '__main__.Example'>,))
    Meta.__new__:
    	 (<class '__main__.Meta'>, 'SubExample', (<class '__main__.Example'>,), {'__module__': '__main__', '__qualname__': 'SubExample', 'a': 10})
    Example.__init_subclass__:
    	(<class '__main__.SubExample'>, {})
    Example.__init__:
    	(<__main__.SubExample object at 0x112b4f0a0>)



```python
class MirrorDict(dict):
    def __missing__(self, key):
        self[key] = key
        return key


class StringEnumMeta(type):
    @classmethod
    def __prepare__(mcs, class_name, bases):
        return MirrorDict()

    def __contains__(cls, value):
        return hasattr(cls, value)

    def __iter__(cls):
        for key in cls.__dict__:
            if not key.startswith("__"):
                yield key


class StringEnum(metaclass=StringEnumMeta):
    pass


class Cities(StringEnum):
    France
    Spain
    Germany


for x in Cities:
    print(x)

```

# Fin.

