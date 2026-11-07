# References in Python

Every Python variable is a name for an object, and `=` never copies: `b = a` makes `b` a second name for the same list. Passing a list to a function or storing it in a dict does the same. `is` tells you whether two names point at one object; `==` compares contents. To copy, use `list(a)` or `a.copy()` (shallow: the outer list only) or `copy.deepcopy(a)` (everything inside too). Numbers, strings and tuples can't be changed, so sharing them is harmless.

```python
import copy

a = [[1, 2], [3]]
b = a                    # the same list
c = list(a)              # a new outer list, the same inner lists
d = copy.deepcopy(a)     # new lists all the way down

a[0].append(9)
print(b[0], c[0], d[0])  # [1, 2, 9] [1, 2, 9] [1, 2]
print(b is a, c is a, c[0] is a[0])   # True False True
```

<!-- from:typescript -->
## Coming from TypeScript
- The same model as JavaScript: lists and dicts are shared, `=` never copies.
- `list(a)` / `dict(d)` are `[...a]` / `{ ...d }`: shallow.
- `copy.deepcopy` is `structuredClone`.
- `is` is `===` for objects; `==` compares contents (JavaScript has no built-in deep `==`).
<!-- /from -->

<!-- from:default -->
## Key points
- `=`, arguments and return values share the object; nothing is copied.
- `list(a)`, `a.copy()`, `dict(d)` and slices `a[:]` are shallow copies.
- `copy.deepcopy` copies nested lists and dicts.
<!-- /from -->

## Common mistake
`*` repeats the reference, not the list:

```python
grid = [[0] * 3] * 2        # two names for one row
grid[0][0] = 9
print(grid)                 # [[9, 0, 0], [9, 0, 0]]

grid = [[0] * 3 for _ in range(2)]   # fixed: a new row per pass
```

`[0] * 3` on its own is fine: the items are numbers.

## In these exercises
`make_grid(rows, cols, fill)` builds rows that don't share. `add_bonus(scores, name, points)` returns a copy of `scores` with one list extended: copy the dict *and* each list (a dict comprehension with `list(values)` does it), and don't touch `scores` itself.
