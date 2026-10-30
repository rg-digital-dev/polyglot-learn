# Maps in Rust

`HashMap<K, V>` from `std::collections` stores key-value pairs. You add entries with `insert`, read with `get` (which returns an `Option<&V>`, since the key may be missing), and update in place with the entry API: `map.entry(key).or_insert(default)` gives a mutable reference to the value, creating it first if needed. Owned keys such as `String` are the usual choice for maps you return from a function. Iteration order is not defined.

```rust
use std::collections::HashMap;

fn tally(colors: &[&str]) -> HashMap<String, i64> {
    let mut counts = HashMap::new();
    for color in colors {
        *counts.entry(color.to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let t = tally(&["red", "blue", "red"]);
    assert_eq!(t.get("red"), Some(&2));
    assert_eq!(t.get("green"), None);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `HashMap` is like `Map`, not a plain object: keys and values each have one fixed type.
- `map.get(k)` returns `Option<&V>` instead of `undefined`; you must handle the `None` case.
- The `entry(...).or_insert(0)` idiom replaces `m.set(k, (m.get(k) ?? 0) + 1)`.
- Two maps with the same contents are `==`, regardless of insertion order.
- Keys are moved into the map, so a `&str` usually needs `.to_string()` first.
<!-- /from -->

<!-- from:default -->
## Key points
- `use std::collections::HashMap;` is required; it is not in the prelude.
- Use `entry(...).or_insert(...)` to read, modify and write a value with one lookup.
- Return owned `String` keys so the map does not borrow from the input.
<!-- /from -->

## Common mistake
`char::is_alphanumeric` and `str::to_lowercase` follow Unicode, so `'é'` counts as a letter. When a spec says "ASCII letters and digits", use the ASCII versions:

```rust
c.is_ascii_alphanumeric()   // instead of c.is_alphanumeric()
c.to_ascii_lowercase()      // instead of c.to_lowercase()
```

## In this exercise
Implement `pub fn word_count(text: &str) -> HashMap<String, i64>`. Keys are lowercase words; empty text returns an empty map.
