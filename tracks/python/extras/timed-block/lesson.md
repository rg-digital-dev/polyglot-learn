# Context managers

The `with` statement works with any **context manager**: an object with `__enter__`, called when the block starts (its return value is what `as` binds), and `__exit__(exc_type, exc, tb)`, called when the block ends, however it ends. `__exit__` receives the exception that's leaving the block, or `None`s; returning `True` tells Python the exception has been handled, and anything else lets it continue. For simple cases, `@contextlib.contextmanager` turns a generator into a context manager: the code before `yield` is the setup, the code after it the cleanup, and a `try` / `finally` around the `yield` makes the cleanup run on errors too.

```python
import time
from contextlib import contextmanager


@contextmanager
def timer(label):
    start = time.perf_counter()
    try:
        yield
    finally:
        print(f"{label}: {time.perf_counter() - start:.2f} s")


class Override:
    """Change a setting for the duration of a block, then restore it."""
    def __init__(self, settings, key, value):
        self.settings, self.key, self.value = settings, key, value

    def __enter__(self):
        self.old = self.settings[self.key]
        self.settings[self.key] = self.value
        return self.settings

    def __exit__(self, exc_type, exc, tb):
        self.settings[self.key] = self.old   # restored even if the block raised
        return False                          # don't swallow exceptions


settings = {"dir": "/home"}
with timer("block"), Override(settings, "dir", "/tmp") as s:
    print(s["dir"])     # /tmp
print(settings["dir"])  # /home
```

<!-- from:typescript -->
## Coming from TypeScript
- `__enter__` / `__exit__` are the `using` protocol, `Symbol.dispose`, with two extras: `as` binds what `__enter__` returns, and `__exit__` sees the exception and may suppress it.
- `@contextmanager` has no TypeScript equivalent: a generator with one `yield` becomes the whole setup-and-cleanup object.
<!-- /from -->

<!-- from:default -->
## Key points
- `__enter__` returns the `as` value; `__exit__` runs on every way out.
- `__exit__` returning `True` suppresses the exception; use it rarely and narrowly.
- `@contextmanager` plus `try` / `finally` around `yield` for the simple cases.
<!-- /from -->

## Common mistake
A `@contextmanager` without `try` / `finally` skips its cleanup on an error:

```python
@contextmanager
def tag(log, name):
    log.append(f"<{name}>")
    yield                    # if the block raises, the next line never runs
    log.append(f"</{name}>")
```

## In this exercise
`timed_block.py` asks for `tag` (with `@contextmanager`), a `Transaction` class that commits or rolls back, and `IgnoreValueErrors`, which swallows only `ValueError`s. The tests use them in `with` blocks that succeed and blocks that raise.
