# Numbers in Python

`int` has no size limit, so integer arithmetic never overflows. `/` always returns a `float` (`6 / 2` is `3.0`); `//` is floor division and `%` its matching modulo, so the remainder takes the sign of the divisor and `a == (a // b) * b + a % b` always holds. `divmod(a, b)` returns both. `round` rounds halves to the even neighbour ("banker's rounding"); the `decimal` module offers the other rules.

```python
from decimal import Decimal, ROUND_HALF_UP

print(-7 // 2, -7 % 2, divmod(-7, 2))   # -4 1 (-4, 1)
print(int(-7 / 2))                       # -3: int() truncates
print(round(0.5), round(1.5), round(2.5))  # 0 2 2
print(Decimal("2.5").quantize(Decimal(1), rounding=ROUND_HALF_UP))  # 3
print(2**64, 2**63 - 1 < 2**64)          # 18446744073709551616 True
```

<!-- from:typescript -->
## Coming from TypeScript
- `//` is `Math.floor(a / b)`, but for exact integers of any size.
- `%` floors too: `-1 % 5` is `4`, not `-1`, and there is no `-0` for ints.
- `round(2.5)` is `2`, while `Math.round(2.5)` is `3`.
- No `BigInt` needed: every `int` is a big int.
<!-- /from -->

<!-- from:default -->
## Key points
- `//` and `%` floor, so they wrap negative numbers the way a clock does.
- `/` is true division and gives a float.
- `round` rounds halves to even; `decimal` has `ROUND_HALF_UP` (away from zero).
- Ints never overflow, so a size limit is something you check yourself.
<!-- /from -->

## Common mistake
Expecting `round` to round halves up:

```python
round(12.5)   # 12, not 13
round(-12.5)  # -12, not -13
round(13.5)   # 14
```

## In these exercises
`wrap_index` and `split_evenly` are where Python's floor division shines: `%` and `divmod` already do the right thing. `round_percent` needs halves rounded away from zero, which `round` doesn't do. `checked_multiply` has to compare the product with `-2**63` and `2**63` itself, because nothing will overflow on its own.
