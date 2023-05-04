from abc import abstractmethod, ABC


class Extend_methods(ABC):
    @abstractmethod
    def to_name(self) -> str:
        pass


class Person(Extend_methods):
    _name: str
    _age: int
    _address: str

    def __init__(self) -> None:
        self._name = "andy"
        self._age = 18
        self._address = "test"

    def to_name(self) -> str:
        return self._name


def to_name(person: "Person") -> str:
    return person._name


def show_name(self):
    print(self._name)


if __name__ == "__main__":
    person = Person()
    print(f"1: Person's name: {to_name(person)}")
    print(f"2: Person's name: {person.to_name()}")
