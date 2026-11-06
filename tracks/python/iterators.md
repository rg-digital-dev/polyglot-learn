# Iterators and comprehensions in Python

Most of Python's looping goes through the iterator protocol: lists, strings, dicts, files and `range` can all be iterated, and functions like `sorted`, `sum`, `zip`, `enumerate` and `dict.items()` take or return iterables. Comprehensions build new collections from them in one expression: `[expr for x in items if cond]` for a list, `{k: v for ...}` for a dict, `{x for ...}` for a set. With round brackets, `(x * 2 for x in items)` is a *generator*: it computes items one at a time, only when asked.

```python
import re

text = "The cat and the hat"
words = [w.lower() for w in re.findall(r"[a-z]+", text, re.IGNORECASE)]
print(words)                                  # ['the', 'cat', 'and', 'the', 'hat']

lengths = {w: len(w) for w in words}
print(lengths)                                # {'the': 3, 'cat': 3, 'and': 3, 'hat': 3}

for i, (w, n) in enumerate(sorted(lengths.items())[:2]):
    print(i, w, n)                            # 0 and 3, then 1 cat 3
print(sum(len(w) for w in words))             # 15, without building a list
```

<!-- from:typescript -->
## Coming from TypeScript
- A list comprehension replaces `.filter(...).map(...)`: `[f(x) for x in xs if p(x)]`.
- `for i, x in enumerate(xs)` replaces `xs.forEach((x, i) => ...)`.
- Generator expressions are lazy, like iterator helpers on `.values()`.
<!-- /from -->

<!-- from:default -->
## Key points
- `[... for ... in ... if ...]`, `{k: v for ...}`, `{x for ...}`.
- Round brackets make a lazy generator.
- `enumerate`, `zip`, `sorted`, `dict.items()` combine freely.
<!-- /from -->

## Common mistake
A generator can only be used once:

```python
squares = (x * x for x in range(4))
print(sum(squares))   # 14
print(sum(squares))   # 0: it's already used up; use a list to read it twice
```

## In this exercise
Implement `top_words(text, n)` in `top-words/top_words.py`: the `n` most frequent words, most frequent first, ties alphabetical. A `Counter` and `sorted` with a tuple key do most of it.
