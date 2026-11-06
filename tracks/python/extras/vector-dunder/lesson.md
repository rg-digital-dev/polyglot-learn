# Special ("dunder") methods

Python's operators and built-ins call methods with double underscores ("dunders") on your objects: `a + b` calls `a.__add__(b)`, `len(x)` calls `x.__len__()`, `repr(x)` calls `x.__repr__()`, `if x:` calls `x.__bool__()`, a `for` loop calls `x.__iter__()`, and `a == b` calls `a.__eq__(b)`. Define them and your class works with the language's syntax like a built-in type. When an operand has a type you don't support, return the special value `NotImplemented` (don't raise): Python then tries the other operand's *reflected* method (`b.__radd__(a)`), and raises a `TypeError` only if both decline. If you define `__eq__`, also define `__hash__` (from the same fields), or instances can't be dict keys.

```python
class Money:
    def __init__(self, cents: int) -> None:
        self.cents = cents

    def __repr__(self) -> str:
        return f"Money({self.cents})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, Money) and self.cents == other.cents

    def __hash__(self) -> int:
        return hash(self.cents)

    def __add__(self, other: object) -> "Money":
        if not isinstance(other, Money):
            return NotImplemented
        return Money(self.cents + other.cents)

    def __radd__(self, other: object) -> "Money":
        return self if other == 0 else NotImplemented   # lets sum() start from 0

    def __lt__(self, other: "Money") -> bool:
        return self.cents < other.cents


prices = [Money(250), Money(180)]
print(sum(prices), min(prices), Money(5) == Money(5))   # Money(430) Money(180) True
```

<!-- from:typescript -->
## Coming from TypeScript
- JavaScript has no operator overloading; in Python `a + b`, `a == b`, `len(a)` and `for x in a` can all be defined by your class.
- `__repr__` is like a custom `toString()` used for debugging output.
- Return `NotImplemented` (a special value) rather than throwing, so Python can try the other operand.
<!-- /from -->

<!-- from:default -->
## Key points
- Operators and built-ins call dunder methods.
- Return `NotImplemented` for unsupported operand types.
- `__eq__` and `__hash__` go together.
- Reflected methods (`__rmul__`, `__radd__`) handle the operand on the right.
<!-- /from -->

## Common mistake
Raising instead of returning `NotImplemented`:

```python
def __mul__(self, k):
    if not isinstance(k, int):
        raise TypeError("no")       # Python never tries the other operand's __rmul__
        return NotImplemented       # fixed
```

## In this exercise
`vector_dunder.py` has a `Vector` class with stubbed dunder methods. Implement them so vectors print, compare, hash, add, scale (`3 * v` too), report their length with `abs`, are falsy at zero, and unpack into `x, y`.
