# Generators

A function whose body contains `yield` is a **generator function**. Calling it doesn't run the body: it returns a generator object, and each `next()` (or each turn of a `for` loop) runs the body up to the next `yield`, hands out that value, and pauses there with all its local variables intact. When the body ends, iteration stops. That makes generators lazy: a generator can describe an endless sequence, and only the items someone asks for are ever computed. `yield from other` hands out every item of another iterable, which makes recursive generators short. The `itertools` module has ready-made building blocks (`islice`, `count`, `chain`, `batched`).

```python
from itertools import islice


def fibonacci():
    a, b = 0, 1
    while True:          # endless, but only computed on demand
        yield a
        a, b = b, a + b


def evens(items):
    for x in items:
        if x % 2 == 0:
            yield x


print(list(islice(fibonacci(), 8)))          # [0, 1, 1, 2, 3, 5, 8, 13]
print(list(islice(evens(fibonacci()), 4)))   # [0, 2, 8, 34]
```

<!-- from:typescript -->
## Coming from TypeScript
- Same idea as `function*` and `yield`, without the star: `yield` in the body is enough.
- `yield from xs` is `yield* xs`.
- `next(gen)` returns the value directly and raises `StopIteration` at the end, instead of `{ value, done }`.
<!-- /from -->

<!-- from:default -->
## Key points
- `yield` makes a function a generator; the body runs only as items are requested.
- A generator can be endless; take what you need with `islice` or a loop that stops.
- `yield from` delegates to another iterable.
- A generator can be used up only once.
<!-- /from -->

## Common mistake
Building a list "to be safe" throws the laziness away:

```python
def naturals():
    return list(range(1, 10**12))   # tries to build a trillion items first
```

## In this exercise
`lazy_pipeline.py` asks for five generator functions. The tests check the results, that `countdown` is lazy, that `take` pulls only as many items as it needs, and that each function really is a generator.
