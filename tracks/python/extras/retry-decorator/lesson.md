# Decorators

Functions are values: you can pass one to another function and get a new one back. A **decorator** is exactly that, a function that takes a function and returns a replacement, usually a `wrapper` that does something before or after calling the original. `@deco` above a `def` is shorthand for `f = deco(f)`. Accept `*args, **kwargs` in the wrapper so it works for any signature, and put `@functools.wraps(fn)` on it so the result keeps the original's name and docstring (and `__wrapped__` points at the original). A decorator *with arguments*, like `@retry(3)`, is a function that returns a decorator.

```python
import functools
import time


def timed(fn):
    @functools.wraps(fn)
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        try:
            return fn(*args, **kwargs)
        finally:
            print(f"{fn.__name__} took {time.perf_counter() - start:.3f} s")
    return wrapper


def repeat(n):                 # a decorator factory
    def decorator(fn):
        @functools.wraps(fn)
        def wrapper(*args, **kwargs):
            return [fn(*args, **kwargs) for _ in range(n)]
        return wrapper
    return decorator


@timed
@repeat(3)
def roll():
    return 4


print(roll(), roll.__name__)   # roll took 0.000 s, then [4, 4, 4] roll
```

<!-- from:typescript -->
## Coming from TypeScript
- A Python decorator is a plain higher-order function: `@deco` just means `f = deco(f)`, and it works on any function, not only class members.
- `*args, **kwargs` forwards every argument, like `(...args) => fn(...args)` plus named ones.
- `functools.wraps` copies the name and docstring; without it every decorated function is called `wrapper`.
<!-- /from -->

<!-- from:default -->
## Key points
- `@deco` above `def f` means `f = deco(f)`.
- The wrapper takes `*args, **kwargs` and calls `fn(*args, **kwargs)`.
- `@functools.wraps(fn)` keeps the original's identity.
- Decorators with arguments are factories: three levels of functions.
<!-- /from -->

## Common mistake
Calling the function while decorating, instead of inside the wrapper:

```python
def shout(fn):
    return fn().upper()           # runs fn once, now; the "decorated" function is a string
def shout(fn):
    @functools.wraps(fn)
    def wrapper(*args, **kwargs):
        return fn(*args, **kwargs).upper()
    return wrapper                # fixed
```

## In this exercise
`retry_decorator.py` asks for `shout`, `count_calls` (which keeps a `calls` counter on the wrapper) and the decorator factory `retry(times)`. The tests decorate small functions of their own and check names, counts and attempts.
