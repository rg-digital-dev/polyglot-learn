# Collections and sorting in Rust

`Vec<T>` is Rust's growable list, and `&[T]` is a borrowed view (a slice) of one. Sorting happens in place on a mutable vector: `sort()` for natural order, `sort_by(|a, b| ...)` with a comparison, or `sort_by_key(...)`. A function that receives `&[T]` cannot change it, so it copies first, for example with `to_vec()`. Lengths and indexes use the `usize` type, so other integer types need converting.

```rust
fn longest_names(names: &[&str]) -> Vec<String> {
    let mut sorted: Vec<String> = names.iter().map(|s| s.to_string()).collect();
    // longest first, ties alphabetical
    sorted.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
    sorted.dedup();
    sorted
}

fn main() {
    let names = ["Ann", "Bartholomew", "Cy", "Ann"];
    assert_eq!(longest_names(&names), vec!["Bartholomew", "Ann", "Cy"]);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `sort()` compares numbers as numbers, not as strings, so `[10, 9]` sorts correctly without a comparator.
- Comparators return an `Ordering` (`Less`, `Equal`, `Greater`) via `a.cmp(b)`, not a negative or positive number.
- Reverse order: compare `b.cmp(a)`, or sort and then call `reverse()`.
- There is no `slice(0, n)` that clamps silently: `v[..n]` panics if `n > v.len()`. Use `truncate(n)` instead.
- `sort()` returns `()`, so you cannot chain on it.
<!-- /from -->

<!-- from:default -->
## Key points
- Copy a borrowed slice with `to_vec()` before sorting.
- `sort` is stable; `sort_unstable` is faster and fine for plain numbers.
- `truncate(n)` shortens a vector and does nothing if it is already short enough.
<!-- /from -->

## Common mistake
Using an `i64` count where a `usize` is required:

```rust
v.truncate(n);            // error: expected `usize`, found `i64`
v.truncate(n as usize);   // fixed (fine here because n is never negative)
```

## In this exercise
Implement `pub fn top_scores(scores: &[i64], n: i64) -> Vec<i64>`, highest first. Return a new vector; the input slice is borrowed and cannot be changed.
