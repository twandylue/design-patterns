from abc import ABC, abstractmethod
from typing import Callable, TypeVar

T = TypeVar("T")


class Pipe_extension(ABC):
    @abstractmethod
    def pipe(self, func: Callable[[T], T]) -> T:
        pass


class Person(Pipe_extension):
    _name: str
    _age: int

    def __init__(self):
        self._name = "andy"
        self._age = 18

    def pipe(self, func: Callable[[T], T]) -> T:
        # TODO: how to specify T type
        result = func(self)
        return result


class Woman(Person):
    def __init__(self):
        self._name = "amy"
        self._age = 20


def print_name(person: Person) -> Person:
    print(f"{person._name}")
    return person


def print_age(person: Person) -> Person:
    print(f"{person._age}")
    return person


if __name__ == "__main__":
    person = Person()
    person.pipe(print_name).pipe(print_age)
    woman = Woman()
    woman.pipe(print_name).pipe(print_age)
