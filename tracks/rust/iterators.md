# Iterators in Rust

An iterator produces values one at a time through its `next()` method. Collections hand one out with `.iter()` (borrowed items), `.into_iter()` (owned items) or methods such as `str::split_whitespace`. Adapters like `map`, `filter` and `take` wrap an iterator in another one and do no work yet; a consuming method such as `collect`, `sum`, `count` or `for_each` drives the whole chain. `collect` builds whatever type you ask for, usually a `Vec` or a `HashMap`, and a `HashMap` turns into a `Vec` of `(key, value)` pairs the same way.

```rust
fn longest_words(text: &str, n: usize) -> Vec<String> {
    let mut words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() > 2)
        .collect();
    // longest first, then alphabetical
    words.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
    words.dedup();
    words.into_iter().take(n).collect()
}

fn main() {
    let text = "The cat sat on the Mat with another cat";
    assert_eq!(longest_words(text, 3), vec!["another", "with", "cat"]);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Array methods in TS run eagerly and build a new array at every step; Rust adapters are lazy and run once, at the end.
- `.collect()` plays the role of the final array; you state the target type with an annotation or `collect::<Vec<_>>()`.
- Sorting is not part of the chain: collect into a `Vec`, call `sort_by`, then continue with `into_iter()`.
- `a.cmp(b).then_with(|| ...)` is the Rust way to write "compare by this, then by that".
<!-- /from -->

<!-- from:default -->
## Key points
- Adapters are lazy; nothing happens until something consumes the iterator.
- `sort_by` with `.then_with(...)` sorts by several keys in one pass, and it is stable.
- `take(n)` stops early and does not fail when there are fewer than `n` items.
<!-- /from -->

## Common mistake
Using `map` for its side effect and never consuming it:

```rust
text.split_whitespace().map(|w| *counts.entry(w).or_insert(0) += 1);
// warning: unused `Map` that must be used ... iterators are lazy and do nothing unless consumed
text.split_whitespace().for_each(|w| *counts.entry(w).or_insert(0) += 1);  // fixed
```

## In this exercise
Implement `pub fn top_words(text: &str, n: i64) -> Vec<String>`: count the words, sort by count descending then word ascending, and keep the first `n`. `take` wants a `usize`, so write `take(n as usize)`.
