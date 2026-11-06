# Closures in Python

Functions are values in Python, and a function defined inside another can use the outer function's variables; it keeps them alive after the outer function returns. Reading an outer variable just works. *Assigning* to it is different: by default an assignment creates a new local variable, so you declare `nonlocal name` to change the outer one. Each call of the outer function creates fresh variables, so every returned function gets its own state. `lambda` writes a one-expression function inline.

```python
from collections.abc import Callable


def make_tagger(prefix: str) -> Callable[[str], str]:
    n = 0

    def tag(name: str) -> str:
        nonlocal n          # without this, n += 1 raises UnboundLocalError
        n += 1
        return f"{prefix}-{n:03d} {name}"

    return tag


bug, feat = make_tagger("BUG"), make_tagger("FEAT")
print(bug("crash"), bug("typo"), feat("dark mode"))   # BUG-001 crash BUG-002 typo FEAT-001 dark mode
double = lambda x: x * 2
print(double(21))   # 42
```

<!-- from:typescript -->
## Coming from TypeScript
- Closures capture variables like in JavaScript, but assigning to a captured variable needs `nonlocal`.
- `lambda x: x * 2` is `x => x * 2`, limited to one expression.
- `Callable[[str], str]` is the type of a function, like `(s: string) => string`.
<!-- /from -->

<!-- from:default -->
## Key points
- Inner functions see the outer function's variables.
- `nonlocal x` to assign to an outer variable.
- Each outer call creates new state.
<!-- /from -->

## Common mistake
Closures created in a loop all see the loop variable's last value:

```python
funcs = [lambda: i for i in range(3)]
print([f() for f in funcs])               # [2, 2, 2]
funcs = [lambda i=i: i for i in range(3)] # fixed: bind the current value
```

## In this exercise
Implement `make_counter(start, step)` in `make-counter/make_counter.py`. Return a function; each call returns the current value, starting with `start`, then advances it by `step`. The tests use two counters at once, so they must not share state.
