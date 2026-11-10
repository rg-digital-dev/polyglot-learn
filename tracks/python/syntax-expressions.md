# Operators and Strings in Python

`==` compares values, and `is` asks whether two names point at the same object. Use `is` for `None` only: `x is None`. Comparisons chain: `1 < 2 < 3` means `1 < 2 and 2 < 3`. The logical operators are the words `and`, `or` and `not`, and `and`/`or` return one of their operands, so `name or "world"` gives a default. Join strings with `+`, but only strings: numbers must be converted. An f-string puts values inside text: `f"Hi, {name}!"`. String methods such as `strip()` and `upper()` return new strings, because strings never change.

```python
name = "  Ada "
print(f"Hi, {name.strip()}!")         # --> Hi, Ada!
print("Hi, " + "Bob")                 # --> Hi, Bob
print(1 < 2 < 3)                      # --> True
print([1] == [1], [1] is [1])         # --> True False
print("" or "world")                  # --> world
print(7 / 2, 7 // 2, 7 % 2)           # --> 3.5 3 1

try:
    "Score: " + 5
except TypeError as e:
    print(e)                          # --> can only concatenate str (not "int") to str
print("Score: " + str(5))             # --> Score: 5
```

<!-- from:typescript -->
## Coming from TypeScript
- `==` already refuses to mix types: `1 == "1"` is `False`. There is no `===`.
- `and`, `or`, `not` replace `&&`, `||`, `!`. The symbol forms are not valid.
- No template backticks: use `f"..."`, with the expression directly inside the braces.
- `"Score: " + 5` is a `TypeError`.
- `name or "world"` replaces `name || "world"`, but `0`, `""` and `[]` also count as false.
<!-- /from -->

<!-- from:php -->
## Coming from PHP
- Join strings with `+`, not `.`. A `.` is attribute access.
- Interpolation needs the `f` prefix: `"Hi {name}"` is plain text.
<!-- /from -->

<!-- from:default -->
## Key points
- `==` for equal values, `is` for identity (and `x is None`).
- `and`, `or`, `not` short-circuit, and `and`/`or` return an operand.
- `/` always gives a float; `//` rounds down; `%` is the remainder.
- Use an f-string or `str(x)` to mix numbers into text.
<!-- /from -->

## Common mistake
Writing `if name is "Ada":`. It may work for some values and fail for others, because `is` compares identity, not content. Use `==` for values and keep `is` for `None`.
