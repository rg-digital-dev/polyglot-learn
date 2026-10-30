# Functions and strings in Rust

A Rust function declares the type of every parameter and its return type: `fn name(x: T) -> R`. The body is a block, and its last expression (with no semicolon) is the return value; `return` is only needed for early exits. Text comes in two shapes: `&str` is a borrowed view of some UTF-8 text, and `String` is an owned, growable buffer. Functions usually take `&str` (so callers can pass either kind) and return `String` when they build something new. `format!` builds a `String` with placeholders like `{name}`.

```rust
pub fn label(item: &str, qty: i64) -> String {
    let item = item.trim();          // &str: a view into the original text
    if qty == 1 {
        return format!("1 {item}");  // early exit
    }
    format!("{qty} {item}s")         // last expression is the result
}

fn main() {
    println!("{}", label("  apple ", 3)); // "3 apples"
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Types are required on parameters and return values; there is no inference across function boundaries.
- `&str` vs `String` has no TS equivalent. Rule of thumb: take `&str`, return `String`.
- `format!("Hi, {name}!")` plays the role of a template literal; `+` also works but needs a `String` on the left.
- Methods like `trim()` return a `&str` pointing into the original text instead of copying it.
- `s.len()` is the length in bytes, not characters: `"Zoë".len()` is 4.
<!-- /from -->

<!-- from:default -->
## Key points
- The last expression of a block is its value; a trailing `;` turns it into a statement that yields `()`.
- Take `&str` for read-only text input, return `String` for newly built text.
- `trim()`, `is_empty()`, `chars()` and friends are methods on `str`; the std docs list them all.
<!-- /from -->

## Common mistake
Adding a semicolon after the final expression:

```rust
fn shout(s: &str) -> String {
    s.to_uppercase();   // error: expected `String`, found `()`
}
```

Drop the `;` so the value is returned: `s.to_uppercase()`.

## In this exercise
Implement `pub fn greet(name: &str) -> String`. Return the full sentence, e.g. `"Hello, Alice!"`, with surrounding whitespace removed and `"world"` used when nothing is left.
