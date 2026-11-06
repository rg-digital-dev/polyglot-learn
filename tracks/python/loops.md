# Loops in Python

Python's `for` loop goes over the items of anything iterable: a list, a string, a dict's keys, or a `range`. `range(1, 10)` counts from 1 up to *but not including* 10, and `range(0, 10, 3)` steps by 3. `while` repeats as long as a condition holds; `break` and `continue` work as usual. For sums and tests over many items, the built-ins `sum()`, `any()` and `all()` take a *generator expression*: `sum(x * x for x in numbers)` reads like the loop it replaces.

```python
total = 0
for n in range(1, 20):
    if n % 3 == 0 or n % 5 == 0:
        total += n
print(total)  # 78

# The same, as one expression:
print(sum(n for n in range(1, 20) if n % 3 == 0 or n % 5 == 0))  # 78

primes = [2, 3, 5, 7]
print(any(p > 5 for p in primes), all(p % 2 for p in primes))   # True False
```

<!-- from:typescript -->
## Coming from TypeScript
- `for x in items:` is `for (const x of items)`; there's no C-style `for (i = 0; ...)`.
- `range(a, b)` replaces counting loops, and excludes `b`.
- `sum`, `any` and `all` with a generator expression replace `reduce`, `some` and `every`.
- `enumerate(items)` gives `(index, item)` pairs when you need the index.
<!-- /from -->

<!-- from:default -->
## Key points
- `for item in iterable:`; `range(start, stop, step)` stops before `stop`.
- `sum(... for x in ...)`, `any(...)`, `all(...)` instead of manual accumulators.
- `enumerate` for an index, `zip` to walk two sequences together.
<!-- /from -->

## Common mistake
Changing a list while looping over it skips items:

```python
items = [1, 2, 2, 3]
for x in items:
    if x == 2:
        items.remove(x)      # leaves one 2 behind
items = [x for x in items if x != 2]   # fixed: build a new list
```

## In this exercise
Implement `sum_of_multiples(factors, limit)` in `sum-of-multiples/sum_of_multiples.py`: add up every number below `limit` that is a multiple of at least one factor, counting each number once and ignoring a factor of 0.
