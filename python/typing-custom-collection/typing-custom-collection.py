"""Custom typed MutableSequence class

In this file we are exploring defining our own MutableSequence class
and the options we have for typing. By Mutable Sequence we mean a class
that has the following methods:
  "__reversed__"
  "__iter__"
  "__len__"
  "__contains__"
  "__getitem__"
  "__setitem__"
  "__delitem__"

Note this file passes mypy and pyright in vscode. 🎉
"""

import random
import time
from collections import abc, deque
from typing import (
    Any,
    Callable,
    Generic,
    Iterable,
    Iterator,
    Protocol,
    SupportsIndex,
    TypeVar,
    overload,
    runtime_checkable,
)

# The `MutableSequence` from collections.abc is defined as an abstract base class
# It subclasses `Sequence` which subclasses `Reversible`, `Collection`.
# `Reversible` and `Collection` define the __subclasshook__ which means
# any class defining "__reversed__", "__iter__" will be considered a Reversible
# and any class defining  "__len__", "__iter__", "__contains__" are a Collection.


# Let's see this in action


def abc_example() -> None:

    class A:
        def __reversed__(self) -> Iterator[Any]:
            return self

        def __iter__(self) -> Iterator[Any]:
            return self

        def __next__(self) -> int:
            return 1

    a = A()

    # See a/A is considered a Reversible
    print(f"{isinstance(a, abc.Reversible)=}")
    print(f"{issubclass(A, abc.Reversible)=}")


# In a way this makes them very like protocols - in that they use structural and not nominal typing
# but they are not protocols. When specifying runtime_checkable protocols work in a very similar way.
# When ever you subclass a Protocol, in the __init_subclass__ method Protocol creates a function
# called _proto_hook and sets it as the __subclasshook__ method.
# __subclasshook__ is not called by default when you call issubclass. __subclasscheck__ is called
# but abc.ABCMeta overrides __subclasscheck__ to call __subclasshook__ if it exists.
# So you might wonder why Protocols work - this is because Protocol also has
# ABCMeta as its metaclass so it also overrides __subclasscheck__ to call __subclasshook__.

# In the example below I monkey patch ABCMeta.__subclasscheck__ to print
# and show this is called when checking subclass for Protocols and ABCs!


def show_workings_of_abc_and_protocol() -> None:
    from abc import ABCMeta

    original_subclasshook: Callable[[ABCMeta, type], bool] = ABCMeta.__subclasscheck__

    def __simons_subclasscheck__(cls: ABCMeta, subclass: type[Any]) -> bool:
        """Override for issubclass(subclass, cls)."""
        print(f"SIMON - running __subclasshook__ on {cls} with subclass {subclass}")
        return original_subclasshook(cls, subclass)

    ABCMeta.__subclasscheck__ = __simons_subclasscheck__  # type: ignore

    class A:
        def __reversed__(self) -> Iterator[Any]:
            return self

        def __iter__(self) -> Iterator[Any]:
            return self

        def __next__(self) -> int:
            return 1

    issubclass(A, abc.Reversible)

    @runtime_checkable
    class ExampleProtocol(Protocol):
        pass

    issubclass(A, ExampleProtocol)

    # Let's fix this back now!
    ABCMeta.__subclasscheck__ = original_subclasshook  # type: ignore


# In contrast, the MutableSequence abc doesn't define any __subclasshook__
# This means that even if you define all the methods, you won't be considered
# a MutableSequence unless you subclass it OR call MutableSequence.register(MyClass)

# So we have two options to define a MutableSequence


# 1. Subclass MutableSequence like:
# class MyMutableSequence1(abc.MutableSequence):
#     """here we must define all the methods of abc.MutableSequence"""
#     ...


# # 2. Register a class with MutableSequence like:
# class MyMutableSequence2:
#     """here we must define all the methods of abc.MutableSequence"""
#     ...
# abc.MutableSequence.register(MyMutableSequence2)

# The issue with the second is that type checkers won't know that MyMutableSequence2
# is a MutableSequence!


# We can instead define our very own MutableSequence class subclassing off protocol
T = TypeVar("T")


@runtime_checkable
class MutableSequenceP(Protocol[T]):
    def __getitem__(self, __key: SupportsIndex) -> T: ...

    def __setitem__(self, __key: SupportsIndex, __value: T) -> None: ...

    def __delitem__(self, __key: SupportsIndex) -> None: ...

    def __len__(self) -> int: ...

    def __iter__(self) -> Iterator[T]: ...

    def __reversed__(self) -> Iterator[T]: ...

    def __contains__(self, __key: object) -> bool: ...


# Example implementation of the protocol
class Bucket[T]():
    def __init__(self, items: Iterable[T]) -> None:
        self._items = list(items)

    def __getitem__(self, __key: SupportsIndex) -> T:
        return self._items[__key]

    def __setitem__(self, __key: SupportsIndex, __value: T) -> None:
        self._items[__key] = __value

    def __delitem__(self, __key: SupportsIndex) -> None:
        del self._items[__key]

    def __contains__(self, __key: object) -> bool:
        return __key in self._items

    def __reversed__(self) -> Iterator[T]:
        yield from reversed(self._items)

    def __len__(self) -> int:
        return len(self._items)

    def __iter__(self) -> Iterator[T]:
        return self._items.__iter__()


# Below we have an implementation of a class inheriting from MutableSequence
# This was quick to get working ignoring static type checking

# This was super painful to get working with mypy/pyright in vscode!
# Note we needed a lot of overloads to get the type checker to work

# On the implementation itself I typed with Any to avoid missing type errors


class BucketInherit(abc.MutableSequence[T]):
    def __init__(self, items: Iterable[T]) -> None:
        self._items = list(items)

    @overload
    def __getitem__(self, index: int) -> T: ...

    @overload
    def __getitem__(self, index: slice) -> abc.MutableSequence[T]: ...

    def __getitem__(self, index: int | slice) -> T | abc.MutableSequence[T]:
        return self._items[index]

    @overload
    def __setitem__(self, index: int, value: T) -> None: ...

    @overload
    def __setitem__(self, index: slice, value: Iterable[T]) -> None: ...

    def __setitem__(self, index: int | slice, value: Any) -> None:
        # the value Any here seems to be an issue on github here:
        # https://github.com/python/mypy/issues/7858
        # Also discussed here
        # https://stackoverflow.com/questions/68473151/mypy-complains-about-setitem-signature
        self._items[index] = value

    @overload
    def __delitem__(self, index: int) -> None: ...

    @overload
    def __delitem__(self, index: slice) -> None: ...

    def __delitem__(self, index: int | slice) -> None:
        del self._items[index]

    def __contains__(self, __key: object) -> bool:
        return __key in self._items

    def __reversed__(self) -> Iterator[T]:
        yield from reversed(self._items)

    def __len__(self) -> int:
        return len(self._items)

    def __iter__(self) -> Iterator[T]:
        return self._items.__iter__()

    def insert(self, index: int, object: T) -> None:
        self._items.insert(index, object)


class IntBucket(Bucket[int]):
    def __init__(self, items: Iterable[int] = [1, 2, 3]) -> None:
        super().__init__(items)


U = TypeVar("U", bound=MutableSequenceP[Any])


# How to type this?
# it only really needs __getitem__ and __setitem__ and __len__
# for now I will just use any subtype of the protocol
def shuffle(l: U) -> U:
    """Shuffles a list"""
    for i in range(len(l)):
        j = random.randint(i, len(l) - 1)
        l[i], l[j] = l[j], l[i]
    return l


def swap_first_to_last(l: U) -> U:
    """Swaps the first and last elements of a list"""
    l[0], l[-1] = l[-1], l[0]
    return l


def bucket_example() -> None:
    """Testing the type checker"""
    r = deque([1, 2, 3, 4])
    y = [1, 2, 3, 4]
    z = list("abc")
    out = shuffle(z)  # try changing z to 1 and see the error

    bucket = Bucket([1, 2, 3])
    int_bucket = IntBucket()
    bucket_inherit = BucketInherit([1, 2, 3])

    # require runtime_checkable
    print(isinstance(bucket, MutableSequenceP))
    print(issubclass(Bucket, MutableSequenceP))

    shuffled_bucket = shuffle(bucket)
    print(shuffled_bucket)
    print(bucket)
    print(len(bucket))
    for x in bucket:
        print(x)


def speed_test_protocol() -> None:
    start = time.perf_counter()
    bucket = Bucket([1, 2, 3])
    for _ in range(1_000_000):
        isinstance(bucket_example, MutableSequenceP)
    end = time.perf_counter()
    print(f"Time taken for protocol: {end - start}")


def speed_test_abc() -> None:
    start = time.perf_counter()
    bucket = BucketInherit([1, 2, 3])
    for _ in range(1_000_000):
        isinstance(bucket_example, abc.MutableSequence)
    end = time.perf_counter()
    print(f"Time taken for abc: {end - start}")


def run_speed_tests() -> None:
    speed_test_protocol()
    speed_test_abc()


def main() -> None:
    abc_example()
    show_workings_of_abc_and_protocol()
    bucket_example()
    run_speed_tests()


if __name__ == "__main__":
    main()


### Generics in python:
from abc import ABCMeta, abstractmethod
from collections.abc import Iterable
from typing import Any, Protocol, TypeVar, runtime_checkable


@runtime_checkable
class SupportsLessThan(Protocol):
    def __lt__(self, __value: Any) -> bool: ...


S = TypeVar("S", bound=SupportsLessThan)


def my_max(items: Iterable[S]) -> None | S:
    items_iter = iter(items)
    _max = next(items_iter, None)
    if _max is None:
        return None
    for item in items_iter:
        if _max < item:
            _max = item
    return _max


print(issubclass(int, SupportsLessThan))


class Human:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    # Try commenting this out and see what happens
    def __lt__(self, other: Any) -> bool:
        return self.age < other.age


example: list[int] = [1, 2, 3]
my_max(example)
my_max(["a", "b", "c"])
my_max((Human("Alice", 29), Human("Bob", 32), Human("Charlie", 31)))
