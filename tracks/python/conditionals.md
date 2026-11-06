# Conditionals in Python

`if`, `elif` and `else` choose between blocks; the conditions are combined with the words `and`, `or` and `not`, which short-circuit like `&&` and `||`. Comparisons can be chained: `0 <= x < 10` means `0 <= x and x < 10`. Every value has a truth value: `0`, `""`, `[]`, `{}` and `None` count as false, everything else as true. A conditional *expression* picks a value in one line: `a if condition else b`. `%` is the remainder, and `//` divides and rounds down.

```python
def grade(score: int) -> str:
    if not 0 <= score <= 100:
        return "invalid"
    elif score >= 90:
        return "A"
    elif score >= 70 and score % 2 == 0:
        return "B (even)"
    return "C"


print(grade(95), grade(72), grade(71), grade(120))  # A B (even) C invalid
print("even" if 12 % 2 == 0 else "odd")             # even
```

<!-- from:typescript -->
## Coming from TypeScript
- `and`, `or`, `not` instead of `&&`, `||`, `!`.
- `elif`, not `else if`, and no parentheses around the condition.
- `a if cond else b` is the ternary `cond ? a : b`, with the order changed.
- `==` compares values (there's no `===`); `is` checks identity and is for `None`.
<!-- /from -->

<!-- from:default -->
## Key points
- `if` / `elif` / `else`, with `and`, `or`, `not`.
- Chained comparisons: `low <= x < high`.
- Empty and zero values are false in a condition.
<!-- /from -->

## Common mistake
Mixing up `/` and `//`:

```python
2024 / 4 == 506.0   # True, but / always gives a float
2024 // 4           # 506: whole-number division
2024 % 4 == 0       # the usual way to test divisibility
```

## In this exercise
Implement `leap_year(year)` in `leap-year/leap_year.py`: divisible by 4, except centuries, except every 400 years. It fits in one boolean expression.
