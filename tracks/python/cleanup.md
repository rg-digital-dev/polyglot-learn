# Cleanup in Python

The `with` statement is Python's cleanup tool. `with thing as x:` calls `thing.__exit__` when the block ends, whichever way it ends: normally, by `return`, or by an exception (which then continues upward). Files, locks and connections all work this way. For a group of resources whose number you don't know in advance, `contextlib.ExitStack` collects them: `stack.enter_context(resource)` adds one, and leaving the `with ExitStack() as stack:` block exits all of them, newest first. `try` / `finally` works too and is what `with` does underneath.

```python
from contextlib import ExitStack


class Lock:
    def __init__(self, name: str) -> None:
        self.name = name

    def __enter__(self) -> "Lock":
        print("lock", self.name)
        return self

    def __exit__(self, *exc: object) -> None:
        print("unlock", self.name)


def take(names: list[str]) -> None:
    with ExitStack() as stack:
        for name in names:
            if not name:
                raise ValueError("empty name")   # unlocks the ones taken so far
            stack.enter_context(Lock(name))


try:
    take(["a", "b", ""])
except ValueError as e:
    print(e)   # lock a, lock b, unlock b, unlock a, then: empty name
```

<!-- from:typescript -->
## Coming from TypeScript
- `with x as r:` is `using r = x`; `__exit__` plays the part of `Symbol.dispose`.
- `ExitStack` is `DisposableStack`: `enter_context` is `use`.
- `try` / `finally` is the same as in TypeScript.
<!-- /from -->

<!-- from:default -->
## Key points
- `with resource as r:` closes it on every way out of the block.
- `ExitStack` for a dynamic number of resources.
- Close in one place only: `with` already calls `close()`.
<!-- /from -->

## Common mistake
Opening several resources in a comprehension without a stack:

```python
opened = [resource.open(n) for n in names]   # if the third open fails, the first two stay open
```

## In this exercise
`resource.open(name)` returns a resource with `read()`, `close()` and `with` support. `read_all` opens them all on an `ExitStack`, then reads each; the stack closes everything however `read_all` ends.
