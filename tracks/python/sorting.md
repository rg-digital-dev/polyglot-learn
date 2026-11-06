# Sorting in Python

`sorted(items)` returns a *new* sorted list and leaves the original alone; `items.sort()` sorts the list in place and returns `None`. Both take `reverse=True` and a `key` function that says what to sort by: `sorted(people, key=lambda p: p.age)`. To sort by several things, return a tuple from the key; tuples compare item by item. Python's sort is stable, so items with equal keys keep their order. Slicing with `[:n]` takes the first `n` items and never fails on a short list.

```python
words = ["pear", "fig", "banana", "kiwi"]

print(sorted(words))                          # ['banana', 'fig', 'kiwi', 'pear']
print(sorted(words, key=len))                 # ['fig', 'pear', 'kiwi', 'banana']
print(sorted(words, key=lambda w: (-len(w), w)))  # longest first, then alphabetical
print(words)                                  # unchanged: sorted() made a copy
print(sorted(words)[:10])                     # fine even with fewer than 10
```

<!-- from:typescript -->
## Coming from TypeScript
- `sorted(xs)` is `xs.toSorted()`; `xs.sort()` sorts in place but returns `None`, not the list.
- No comparator returning -1/0/1: give a `key` function instead, and `reverse=True` for descending.
- Numbers sort as numbers by default (no `[10, 9].sort()` surprise).
<!-- /from -->

<!-- from:default -->
## Key points
- `sorted()` copies, `.sort()` changes the list.
- `key=` to choose what to compare; a tuple key for ties.
- The sort is stable.
<!-- /from -->

## Common mistake
Sorting the caller's list by accident:

```python
def top(scores, n):
    scores.sort(reverse=True)   # the caller's list is now reordered
    return scores[:n]
def top(scores, n):
    return sorted(scores, reverse=True)[:n]   # fixed
```

## In this exercise
Implement `top_scores(scores, n)` in `top-scores/top_scores.py`: the `n` highest scores, highest first. The tests check that the input list is unchanged afterwards.
