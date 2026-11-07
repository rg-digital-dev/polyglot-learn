# Error strategies in Python

`key in settings` answers "is it there?" on its own; `settings.get(key, fallback)` combines it with picking a default. Keep the conversion outside any `try` that falls back, and be careful with `int()`: it is generous. It accepts surrounding whitespace, a sign, underscores between digits (`int("8_080")` is `8080`) and digits from other scripts, and `str.isdigit()` accepts even more (`"²".isdigit()` is `True`). Compare against `"0"`–`"9"` yourself when only ASCII digits are allowed.

Python 3.11 added `ExceptionGroup` for several errors at once, with `except*` to handle each kind. Its `str()` is only its message plus a count, so the message should list the parts:

```python
problems = []
if len(name) < 3:
    problems.append("name: too short")
if "@" not in email:
    problems.append("email: missing @")
if problems:
    raise ExceptionGroup("; ".join(problems), [ValueError(p) for p in problems])
```

<!-- from:typescript -->
## Coming from TypeScript
- `key in d` checks only the dict's own keys: there's no prototype, so `"constructor"` is just a key.
- `ExceptionGroup` is `AggregateError` with the same catch: `str(eg)` is `"message (2 sub-exceptions)"`, not the parts.
- `int()` throws `ValueError` instead of returning `NaN`, but it accepts more than `/^[0-9]+$/` does.
<!-- /from -->

<!-- from:default -->
## Key points
- `key in d` for "is it there?"; then validate the value separately.
- `int()` accepts `" 7 "`, `"+7"` and `"8_080"`; check the characters first if the format is stricter.
- `ExceptionGroup` groups errors, but its `str()` doesn't list them.
<!-- /from -->

## Common mistake
A bare `except` that turns every problem into the default:

```python
try:
    return int(settings[key])
except Exception:
    return fallback  # "eighty" quietly becomes the fallback
```

## In these exercises
`int_setting` checks `key in settings`, then the characters, and raises `ValueError` with the key in the message. `validate_signup` collects the problems in a list and raises once, with all of them in the message.
