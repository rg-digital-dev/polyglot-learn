# Errors in Python

Python reports failures with exceptions: `raise ValueError("not a number")` stops the function and travels up the call stack until a `try` / `except` catches it. Pick the built-in class that fits: `ValueError` for a value of the right type but wrong content, `TypeError` for the wrong type, `KeyError` for a missing key. Catch only what you expect (`except ValueError:`), not everything. Built-in parsers like `int()` raise `ValueError` for bad text, but they're more lenient than you might want: `int(" +80 ")` is `80`.

```python
def parse_percent(text: str) -> int:
    s = text.strip().removesuffix("%")
    if not s.isascii() or not s.isdigit():
        raise ValueError(f"not a number: {text!r}")
    n = int(s)
    if n > 100:
        raise ValueError(f"over 100: {n}")
    return n


try:
    parse_percent("12x%")
except ValueError as e:
    print(e)                    # not a number: '12x%'
print(parse_percent(" 42% "))   # 42
```

<!-- from:typescript -->
## Coming from TypeScript
- `raise` and `try` / `except` instead of `throw` and `try` / `catch`.
- `except ValueError as e:` catches by class, which TypeScript can't do in the `catch` itself.
- Throw exception objects, never strings: `raise "oops"` is itself an error.
- `int("abc")` raises instead of returning `NaN`.
<!-- /from -->

<!-- from:default -->
## Key points
- `raise ValueError("message")` for bad input.
- Catch specific classes: `except ValueError as e:`.
- Validate shape first; `int()` accepts signs, spaces and underscores.
<!-- /from -->

## Common mistake
Catching everything hides real bugs:

```python
try:
    port = parse_port(text)
except Exception:   # also swallows typos (NameError) and your own bugs
    port = 80
```

## In this exercise
Implement `parse_port(input)` in `parse-port/parse_port.py`. Raise `ValueError` for anything that isn't a plain number from 1 to 65535. Note that `"٣".isdigit()` is true for non-ASCII digits, so check the characters are ASCII digits.
