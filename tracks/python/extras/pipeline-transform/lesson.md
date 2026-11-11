# Pipeline steps with generator expressions and itertools

Each step of a pipeline takes an iterable and returns an iterator. Steps then plug into each other like shell commands joined by `|`, and since each one is lazy, items flow through one at a time:

```python
import itertools

def evens(numbers):
    return (n for n in numbers if n % 2 == 0)     # a generator expression

def squares(numbers):
    return (n * n for n in numbers)

pipeline = squares(evens(itertools.count()))      # endless, and that's fine
print(list(itertools.islice(pipeline, 4)))        # [0, 4, 16, 36]

it = iter("abcdefg")
while chunk := list(itertools.islice(it, 3)):     # take 3 at a time from one iterator
    print(chunk)                                  # ['a','b','c'] ['d','e','f'] ['g']

prices = [3, 1, 4]
print(list(itertools.accumulate(prices)))         # [3, 4, 8]: running sums
a, b = itertools.tee(iter(prices))                # two independent copies of one iterator
print(list(zip(a, itertools.accumulate(b))))      # [(3, 3), (1, 4), (4, 8)]
```

`itertools` holds the building blocks: `islice` takes a slice of any iterator, `accumulate` keeps a running result, `tee` splits one iterator into several, and `count` and `cycle` make endless ones. The `:=` operator assigns and tests in one go, which suits "take a batch while there is one".

<!-- from:typescript -->
## Coming from TypeScript
- A generator expression `(x for x in xs if …)` is `xs.filter(…).map(…)`, but lazy: there are no intermediate arrays, and it works on endless inputs.
- `itertools.islice(it, n)` is like `take(n)` from a library such as iter-tools. Python ships these in the standard library.
- An iterator can be read only once, like a JS iterator. `tee` is how you read it twice.
<!-- /from -->

<!-- from:default -->
## Key points
- Return generators (or generator expressions) so steps stay lazy and chain.
- Use `itertools` instead of index arithmetic: `islice`, `accumulate`, `tee`.
- An iterator is used up once read. Make a list only when you really need everything at once.
<!-- /from -->

## Common mistake
Using an iterator twice. `total = sum(o.total for o in orders)` reads `orders` to the end, so a second loop over the same generator sees nothing at all, and no error tells you so. Use `tee`, or make a list if the data is small.

## In this stage
In `transform.py`, implement `between`, `in_category`, `batched` and `running_total`. All four must be lazy.
