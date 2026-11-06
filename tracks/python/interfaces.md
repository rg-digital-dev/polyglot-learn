# Interfaces in Python

Python is duck-typed: any object with the right methods can be used where those methods are called, with no declaration needed. To give that shape a name for type checkers, write a `Protocol`: a class listing the methods, with `...` as their bodies. Classes don't have to mention the protocol to match it. Methods take `self` explicitly as their first parameter, and instance fields are set in `__init__`. Different classes in one list are fine as long as they share the methods you call.

```python
from typing import Protocol


class Animal(Protocol):
    def sound(self) -> str: ...


class Dog:
    def sound(self) -> str:
        return "woof"


class Cow:
    def __init__(self, name: str) -> None:
        self.name = name

    def sound(self) -> str:
        return f"{self.name} says moo"


def chorus(animals: list[Animal]) -> str:
    return ", ".join(a.sound() for a in animals)


print(chorus([Dog(), Cow("Bella")]))   # woof, Bella says moo
```

<!-- from:typescript -->
## Coming from TypeScript
- A `Protocol` is a structural interface, like a TypeScript `interface`: matching methods are enough.
- `self` is written out as the first parameter and used for every field: `self.name`, never a bare `name`.
- The constructor is `__init__`, and there's no `new`: `Cow("Bella")`.
<!-- /from -->

<!-- from:default -->
## Key points
- Duck typing: having the methods is what counts.
- `Protocol` names the shape for type checkers; classes don't inherit from it.
- `self` is the first parameter of every method.
<!-- /from -->

## Common mistake
Forgetting `self`:

```python
class Square:
    def __init__(self, side): self.side = side
    def area(self): return side * side        # NameError: side
    def area(self): return self.side * self.side   # fixed
```

## In this exercise
`shapes/shapes.py` declares the `Shape` protocol and the classes `Rect` and `Square`. Implement their `area` and `label` methods, then `describe_all`, largest area first, keeping equal areas in their input order and leaving the input list alone.
