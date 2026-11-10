# Types and Annotations in Python

Python is dynamic: values carry their type (`type(x)`), and names do not. You may add annotations such as `n: int = 1` or `def f(n: int) -> int`, but the interpreter ignores them. A tool such as mypy or your editor can check them; nothing does when the program runs. Calling an `int` function with strings runs fine if the operations work on strings. The basic types are `int`, `float`, `str`, `bool` and `None`. An `int` has no size limit. Python never converts between text and numbers for you.

```python
def add(a: int, b: int) -> int:
    return a + b

print(add(2, 3))              # --> 5
print(add("x", "y"))          # --> xy   (the hints are not checked)

n: int = 1
n = "text"                    # runs; a type checker would complain
print(type(n).__name__)       # --> str

print(type(1).__name__, type(1.0).__name__, type(None).__name__)  # --> int float NoneType

try:
    "a" + 1
except TypeError as e:
    print(e)                  # --> can only concatenate str (not "int") to str
```

<!-- from:typescript -->
## Coming from TypeScript
- Annotations are not checked by anything you run here, and there is no `tsc` step. Python runs code that a checker would reject.
- `int` and `float` are separate types, unlike `number`. `7 / 2` is `3.5` and `7 // 2` is `3`.
- `None` replaces both `null` and `undefined`.
- `"2" + 3` is a `TypeError`, not `"23"`.
<!-- /from -->

<!-- from:php -->
## Coming from PHP
- Even `int` parameters are not checked, and there is no `strict_types` switch.
- `"10" + 1` is a `TypeError`, not `11`. Convert with `int("10")`.
<!-- /from -->

<!-- from:default -->
## Key points
- Types belong to values; names can point at any type.
- Annotations document your intent. They are not checked when the code runs.
- The basics: `int`, `float`, `str`, `bool`, `None`.
- Convert on purpose: `int("5")`, `str(5)`, `float("1.5")`.
<!-- /from -->

## Common mistake
Trusting a hint to reject bad input. `def f(n: int)` accepts a string, and the failure shows up later, somewhere else. Test with the real values the exercise gives you.
