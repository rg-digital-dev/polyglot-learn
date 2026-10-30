# Collections and sorting

Lists are the everyday collection: ordered, indexable, growable. Sorting them looks simple, but languages differ in four ways:

- **In place or a new list.** JS `sort()`, Rust's `sort()`, Go's `slices.Sort`, PHP's `sort()` and Lua's `table.sort` change the list you pass them. Some languages also offer a copying version, such as JS `toSorted()`.
- **Default order.** JavaScript's `sort()` with no comparator compares numbers **as strings**: `[10, 9, 1].sort()` gives `[1, 10, 9]`.
- **Comparator shape.** A comparator either returns a number (negative, zero or positive), an `Ordering` (Rust), or a "less than" boolean (Lua, and Go's older `sort.Slice`).
- **Slicing past the end.** Taking the first `n` items when there are fewer than `n` returns what's there in JS, PHP and Lua, but panics in Rust and Go if you slice without checking.

```text
top_scores([70, 50, 90, 30, 90, 100], 3)  ->  [100, 90, 90]
top_scores([5], 3)                        ->  [5]
```

To sort descending, either pass a reversed comparator (`b - a`, `b.cmp(a)`, `a > b`) or sort ascending and then reverse.

## Don't change the caller's list

A function that sorts its argument in place surprises whoever called it. Copy first: `[...xs]`, `xs.to_vec()`, `slices.Clone(xs)`. PHP arrays are passed by value, so the function already works on a copy. In Lua, copy into a new table.

## Common mistake

Using a comparator that returns `true` for equal items, like `a >= b` in Lua. That breaks the "strict ordering" rule a sort algorithm relies on, and Lua may raise "invalid order function for sorting". Use `a > b`.

## Go deeper

- Exercism's language-neutral spec: [high-scores](https://github.com/exercism/problem-specifications/tree/main/exercises/high-scores)
- Why sort stability matters: [Sorting algorithm: stability (Wikipedia)](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability)
