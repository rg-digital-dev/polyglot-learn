# Functions in Python

A function is `def name(params):` followed by an indented body. Type hints (`a: int`, `-> int`) are optional and not enforced. `return` hands back a value, and a function that reaches the end without `return` gives back `None`. Parameters can have defaults (`b=1`), and callers can pass arguments by name. `return q, r` returns a tuple, which the caller can unpack: `q, r = f()`. A function without a name is `lambda a, b: a + b`, limited to one expression. Functions are values: you can pass them around.

```python
def add(a: int, b: int = 1) -> int:
    return a + b

def divide(a: int, b: int) -> tuple[int, int]:
    return a // b, a % b

q, r = divide(17, 5)
print(add(2), add(2, 3), add(b=10, a=1))  # --> 3 5 11
print(q, r)                               # --> 3 2

def shout(text: str) -> None:
    print(text.upper())                   # prints, but returns None

print(shout("hi"))                        # --> HI   then None
```

<!-- from:typescript -->
## Coming from TypeScript
- `def` replaces `function`, a `:` and indentation replace `{ }`, and `->` replaces the `:` before the return type.
- Hints are not checked, so `add("x", "y")` runs.
- Several results come back as a tuple, not an array: `return q, r`.
- `lambda` is limited to one expression, with no block body.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- The last expression is not returned. Write `return`, or you get `None`.
- Default values and keyword arguments exist: `f(b=2)`.
<!-- /from -->

<!-- from:default -->
## Key points
- `def name(a: int, b: int = 1) -> int:` and an indented body.
- No `return` means `None`.
- Several results are a tuple: `return q, r`, unpacked as `q, r = f()`.
- Pass by name: `f(b=2, a=1)`.
<!-- /from -->

## Common mistake
Using `print` instead of `return`. The tests read the returned value, so a function that prints the right answer and returns `None` fails.
