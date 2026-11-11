# Filters and stats with iterators and maps

Choosing entries is a `filter`, and adding them up is a `fold`. Rust's iterator methods chain like a pipeline and compile down to a plain loop:

```rust
use std::collections::{BTreeMap, HashMap};

fn main() {
    let words = ["apple", "Avocado", "banana", "apple", "cherry", "banana", "apple"];

    // Keep words starting with "a" in any case, and count each one.
    let counts: HashMap<&str, usize> = words
        .iter()
        .filter(|w| w.to_lowercase().starts_with('a'))
        .fold(HashMap::new(), |mut m, w| {
            *m.entry(*w).or_default() += 1;
            m
        });

    // Sort by count, most first, then by name.
    let mut top: Vec<(&str, usize)> = counts.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    println!("{top:?}"); // [("apple", 3), ("Avocado", 1)]

    // A BTreeMap keeps its keys sorted, so printing it is always in order.
    let lengths: BTreeMap<usize, usize> = words.iter().fold(BTreeMap::new(), |mut m, w| {
        *m.entry(w.len()).or_default() += 1;
        m
    });
    println!("{lengths:?}");
}
```

`entry(key).or_default()` returns a mutable reference to the value, inserting the default (0 here) first if the key is missing. Counting needs no `if contains_key`.

`Option::is_none_or(|x| …)` is true for `None`, and otherwise asks the closure. That's exactly how an optional filter behaves.

<!-- from:typescript -->
## Coming from TypeScript
- `filter` and `fold` are `filter` and `reduce`, but lazy: there are no intermediate arrays.
- `entry(k).or_default()` replaces `map.set(k, (map.get(k) ?? 0) + 1)`.
- `sort_by` with `then_with` is a comparator that returns `a.count - b.count || a.name.localeCompare(b.name)`. It works on an `Ordering` instead of numbers.
<!-- /from -->

<!-- from:default -->
## Key points
- Chain `filter`, `map` and `fold`; nothing runs until something consumes the chain.
- `BTreeMap` for sorted keys, `HashMap` for fast lookups.
- `merge` takes `other` by value, so it can move the strings in without cloning.
<!-- /from -->

## Common mistake
Lower-casing the search text again for every entry is fine at this size, but lower-casing only one side isn't. `"DB"` must find `db`, so both the needle and the haystack need `to_lowercase()`.

## In this stage
In `src/filter.rs`, implement `Filter::from_args` and `matches`, then `Stats::add`, `merge` and `top_targets`, and `summarize`.
