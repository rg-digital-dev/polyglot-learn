# Records with dataclasses and Decimal

This project builds a data pipeline for a small shop. Orders come in as CSV and JSON files, get checked, cleaned and added up, and end in a SQLite database and a Markdown report. It's one folder of modules that grows over eight stages, and each stage adds one module.

It starts with the record itself. A dataclass writes `__init__`, `__repr__` and `__eq__` for you from the type annotations. `frozen=True` also makes it read-only and hashable, so it can go in a set or be a dictionary key:

```python
from dataclasses import dataclass, FrozenInstanceError
from datetime import date
from decimal import Decimal

@dataclass(frozen=True)
class Payment:
    payer: str
    day: date
    amount: Decimal

    @property
    def with_tax(self) -> Decimal:
        return self.amount * Decimal("1.21")

p = Payment("Ada", date.fromisoformat("2026-03-14"), Decimal("19.99"))
print(p)                 # Payment(payer='Ada', day=datetime.date(2026, 3, 14), amount=Decimal('19.99'))
print(p.with_tax)        # 24.1879
print(p == Payment("Ada", date(2026, 3, 14), Decimal("19.99")))  # True
try:
    p.amount = Decimal(0)
except FrozenInstanceError:
    print("read-only")

print(0.1 + 0.2)                        # 0.30000000000000004
print(Decimal("0.1") + Decimal("0.2"))  # 0.3
```

Money is a `Decimal` because floats are binary fractions, and `0.1` can't be stored exactly in one. Build a `Decimal` from a string, never from a float: `Decimal(0.1)` copies the float's error along with it.

Raw data is always messier than the code expects. A parser that turns each problem into a `ValueError` naming the field lets later stages report "line 7: quantity: 'two' isn't a whole number" instead of crashing.

<!-- from:typescript -->
## Coming from TypeScript
- A frozen dataclass is a class with `readonly` fields, plus structural `==` and a hash for free. The closest TypeScript has is a `Readonly<{...}>` object literal, but `===` on those compares identity, not contents.
- JavaScript has only floating-point `number`. `Decimal` is what you'd add a library like decimal.js for, and it's built in.
- Python raises instead of returning `NaN` or `Invalid Date`: `int("two")` and `date.fromisoformat("2026-13-01")` both raise `ValueError`.
<!-- /from -->

<!-- from:default -->
## Key points
- `@dataclass(frozen=True)` gives value semantics: compared by content, read-only, hashable.
- Use `Decimal(str)` for money. Decimal arithmetic stays exact.
- Validate at the edge: check and convert each field once, and raise `ValueError` naming the field.
<!-- /from -->

## Common mistake
Converting the price with `float(...)` "for now". Sums of floats drift (`sum([0.1] * 10)` is `0.9999999999999999`), so totals in the report end up a cent off. Once it's a float the error is already there, and no later rounding fully hides it.

## In this stage
In `records.py`, make `Order` a frozen dataclass, implement `total`, and write `parse_order`. The docstring lists the rules. Each later stage adds its own module to this folder.
