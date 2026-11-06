# Dicts in Python

A `dict` maps keys to values: `{"a": 1}`, `d[key]` to read (a `KeyError` if it's missing), `d[key] = value` to write, `key in d` to test. `d.get(key, default)` reads with a fallback, which makes counting short: `counts[w] = counts.get(w, 0) + 1`. Dicts keep their keys in insertion order. `collections.Counter` is a dict made for counting, and `collections.defaultdict(list)` creates missing values for you. Iterate with `for key, value in d.items():`.

```python
from collections import Counter

colors = ["red", "blue", "red", "green", "red"]

counts: dict[str, int] = {}
for c in colors:
    counts[c] = counts.get(c, 0) + 1
print(counts)                       # {'red': 3, 'blue': 1, 'green': 1}

print(Counter(colors).most_common(1))   # [('red', 3)]
for color, n in counts.items():
    print(color, n)
```

<!-- from:typescript -->
## Coming from TypeScript
- A `dict` is a `Map` with object-literal syntax: any hashable value can be a key, and order is kept.
- `d[key]` raises `KeyError` instead of returning `undefined`; use `d.get(key)` for a `None` fallback.
- `key in d` tests keys, like `map.has(key)`.
- `{**a, **b}` merges, like `{ ...a, ...b }`.
<!-- /from -->

<!-- from:default -->
## Key points
- `d.get(key, default)` reads safely; `d[key]` raises for a missing key.
- `Counter` for counting, `defaultdict` for grouping.
- `for k, v in d.items():` to walk keys and values.
<!-- /from -->

## Common mistake
Reading a key before it exists:

```python
counts = {}
counts["red"] += 1                        # KeyError: 'red'
counts["red"] = counts.get("red", 0) + 1  # fixed
```

## In this exercise
Implement `word_count(text)` in `word-count/word_count.py`: count each word (a run of ASCII letters and digits), case-insensitively. The `re` module's `re.findall` is handy for finding the words.
