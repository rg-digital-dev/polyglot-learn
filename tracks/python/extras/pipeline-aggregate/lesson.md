# Aggregating with Counter, defaultdict and sort keys

Adding things up by key is so common that `collections` has two dictionaries for it. A `Counter` treats a missing key as 0, and a `defaultdict(factory)` fills a missing key with `factory()`:

```python
from collections import Counter, defaultdict
from decimal import Decimal

sales = [("books", Decimal("12.50")), ("games", Decimal("60")), ("books", Decimal("7.50"))]

totals = Counter()
for category, amount in sales:
    totals[category] += amount              # no "if category in totals"
print(totals)                               # Counter({'games': Decimal('60'), 'books': Decimal('20.00')})

by_letter = defaultdict(list)
for category, _ in sales:
    by_letter[category[0]].append(category)
print(dict(by_letter))                      # {'b': ['books', 'books'], 'g': ['games']}

# Largest first, ties by name: negate the number, keep the name as it is.
ranked = sorted(totals.items(), key=lambda kv: (-kv[1], kv[0]))
print(ranked)                               # [('games', Decimal('60')), ('books', Decimal('20.00'))]
print(dict(ranked))                         # dicts keep insertion order
```

A sort key that returns a tuple sorts by the first element, then by the next one on a tie. Negating the amount reverses that part of the order only, which `reverse=True` can't do, because it would reverse the names too.

Since Python 3.7, dictionaries keep the order keys were inserted. A dict built from a sorted list is therefore an ordered result you can print as it is.

<!-- from:typescript -->
## Coming from TypeScript
- `totals[k] += v` on a Counter replaces `totals.set(k, (totals.get(k) ?? 0) + v)`.
- `sorted(items, key=…)` sorts by a computed key; JavaScript's `sort` needs a comparator `(a, b) => …` instead. Python compares tuples element by element, so `(-amount, name)` does what `b.amount - a.amount || a.name.localeCompare(b.name)` does.
- Python dicts keep insertion order like a JS `Map`, not like object keys, which put integer-like keys first.
<!-- /from -->

<!-- from:default -->
## Key points
- `Counter` for sums and counts, `defaultdict` for grouping.
- Sort with a key that returns a tuple; negate a number to sort it largest first.
- A function that may get a generator and needs it more than once should start with `list(...)`.
<!-- /from -->

## Common mistake
`sum(o.total for o in orders)` on an empty list returns the int `0`, not `Decimal(0)`. Pass the start value: `sum((…), Decimal(0))`, so an empty summary still compares and formats as money.

## In this stage
In `aggregate.py`, implement `revenue_by`, `top_customers`, `monthly_revenue` and `summarize`. `Summary` is given.
