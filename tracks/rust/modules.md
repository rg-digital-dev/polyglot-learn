# Modules in Rust

A crate is split into modules, and every item inside a module is private unless it is marked `pub`. A module can be written inline as `mod name { ... }`, or declared with `mod name;` and kept in its own file: `pub mod words;` in `src/lib.rs` tells the compiler to load `src/words.rs`. Files are not picked up automatically; only declared modules exist. Paths use `::`, and `crate::` starts from the root of the current crate. A `use` declaration brings a path into scope so you can write the short name.

```rust
mod units {
    pub fn to_kib(bytes: i64) -> i64 {
        round_up(bytes, 1024)
    }

    // private: only code inside `units` can call it
    fn round_up(value: i64, size: i64) -> i64 {
        (value + size - 1) / size
    }
}

use units::to_kib;

fn main() {
    assert_eq!(to_kib(1), 1);
    assert_eq!(units::to_kib(2048), 2);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `pub` is the equivalent of `export`; without it a function is private to its module.
- A file becomes a module only when a parent declares it with `mod`, unlike TS where every file is a module.
- `use crate::words::split;` is roughly `import { split } from "./words"`, but the path starts from the crate root, not from the current file.
- Code outside the crate, such as the tests, reaches items through public paths like `slugify::words::split`.
<!-- /from -->

<!-- from:default -->
## Key points
- `mod words;` in `lib.rs` loads `src/words.rs`; `pub mod` also makes the module visible outside the crate.
- Mark each function you want to call from elsewhere with `pub`.
- `use crate::path::item;` shortens names; it does not load anything by itself.
<!-- /from -->

## Common mistake
Forgetting `pub` on the helper:

```rust
fn split(text: &str) -> Vec<String> { ... }       // in src/words.rs
use crate::words::split;                           // error[E0603]: function `split` is private
pub fn split(text: &str) -> Vec<String> { ... }   // fixed
```

## In this exercise
Implement `pub fn split(text: &str) -> Vec<String>` in `src/words.rs` (its tests run first), then `pub fn slugify(title: &str) -> String` in `src/lib.rs`, which already starts with `pub mod words;`. Add `use crate::words::split;` and reuse it rather than splitting the text again.
