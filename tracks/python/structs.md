# Records in Python

For plain data with named fields, Python has **dataclasses**: put `@dataclass` above a class with annotated fields, and you get a constructor, a readable `repr`, and `==` that compares field by field. The type hints are required here (they're how the fields are found), though Python still doesn't check them. Create one with positional or keyword arguments, read fields with a dot. `@dataclass(frozen=True)` makes instances read-only. Integer arithmetic stays exact: `//` divides and rounds down.

```python
from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int


@dataclass
class Box:
    corner: Point
    width: int
    height: int


b = Box(Point(1, 2), width=10, height=5)
print(b)                           # Box(corner=Point(x=1, y=2), width=10, height=5)
print(b.width * b.height)          # 50
print(Point(1, 2) == Point(1, 2))  # True: compared by value
```

<!-- from:typescript -->
## Coming from TypeScript
- A dataclass is a class *and* a type: there's no separate `interface` for the shape.
- `==` compares fields, unlike `===` on two objects.
- Fields are set by the generated constructor, so there's no `{ x: 1, y: 2 }` literal: write `Point(x=1, y=2)`.
<!-- /from -->

<!-- from:default -->
## Key points
- `@dataclass` plus annotated fields gives `__init__`, `__repr__` and `__eq__`.
- Construct with positional or keyword arguments.
- `frozen=True` for immutable records.
<!-- /from -->

## Common mistake
A list as a default value is rejected, because it would be shared:

```python
@dataclass
class Cart:
    items: list[str] = []                             # ValueError: mutable default
    items: list[str] = field(default_factory=list)    # fixed (from dataclasses import field)
```

## In this exercise
`order-total/order_total.py` defines `LineItem` and `OrderSummary` as dataclasses. Implement `order_total(items, discount_percent)`; the tests compare the returned `OrderSummary` with `==`.
