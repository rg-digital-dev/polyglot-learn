# Error context in Python

Python chains exceptions for you. Inside an `except` block, `raise NewError(...) from e` makes `e` the new exception's `__cause__`; tracebacks then show both, joined by "The above exception was the direct cause of the following exception". So adding context means catching the low-level exception where you know something useful (which key, which file), and raising a new one whose message adds that, `from` the original. Keep the original's text in your message (`f"{key}: {e}"`) so the message alone tells the whole story.

```python
def parse_age(text: str) -> int:
    if not text.isdigit():
        raise ValueError(f"not a number: {text!r}")
    return int(text)


def read_age(form: dict[str, str], field: str) -> int:
    if field not in form:
        raise ValueError(f"{field}: missing")
    try:
        return parse_age(form[field])
    except ValueError as e:
        raise ValueError(f"{field}: {e}") from e


try:
    read_age({"age": "old"}, "age")
except ValueError as e:
    print(e)                          # age: not a number: 'old'
    print(type(e.__cause__).__name__) # ValueError: the original
```

<!-- from:typescript -->
## Coming from TypeScript
- `raise New(...) from e` is `throw new Error(..., { cause: e })`; the cause is `e.__cause__`.
- Even without `from`, an exception raised inside `except` remembers the one being handled (`__context__`).
- `except ValueError as e` filters by class; let other exceptions pass through.
<!-- /from -->

<!-- from:default -->
## Key points
- `raise ValueError(f"{context}: {e}") from e` adds context and keeps the cause.
- Put the `try` around only the call you're wrapping.
- `raise ... from None` deliberately hides the cause (rarely what you want).
<!-- /from -->

## Common mistake
Replacing the message, so the reason is gone:

```python
except ValueError:
    raise ValueError("bad config")          # which key? why?
except ValueError as e:
    raise ValueError(f"{key}: {e}") from e  # fixed
```

## In these exercises
`numbers.parse_number` raises `ValueError`s that say why. `load_setting` finds the key, calls it inside a small `try`, and raises a new `ValueError` with the key in front, `from` the original.

`parse_config` needs the line number in its errors: `enumerate(text.split("\n"), start=1)` numbers every line, blank or not. Use `partition("=")`, not `k, v = line.split("=")`: on `url = a=b` that raises `ValueError: too many values to unpack`, an error with no line number.
