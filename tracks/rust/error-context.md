# Error context in Rust

A `Result` passed up with `?` reaches the caller unchanged. To add context on the way, transform the error first: `map_err` turns it into a new one, usually a message that puts what this function knows in front of the old message. With `Result<T, String>` that's a `format!`. In larger programs you'd define an error enum with a variant that *holds* the lower error and return it from `source()` (see `error-conversion` in Rust in Practice), or use a crate like `anyhow`, whose `.context("...")` does this in one call.

```rust
fn read_age(form: &[(String, String)], field: &str) -> Result<u8, String> {
    let raw = form
        .iter()
        .find(|(k, _)| k == field)
        .map(|(_, v)| v)
        .ok_or_else(|| format!("{field}: missing"))?;
    raw.trim().parse::<u8>().map_err(|e| format!("{field}: {e}"))
}

fn main() {
    let form = vec![("age".to_string(), "old".to_string())];
    assert_eq!(read_age(&form, "age").unwrap_err(), "age: invalid digit found in string");
    assert_eq!(read_age(&form, "height").unwrap_err(), "height: missing");
}
```

<!-- from:typescript -->
## Coming from TypeScript
- There's no `cause` on a `String` error: the context lives in the message. With a custom error type, a field plus `source()` plays the role of `cause`.
- `map_err` is the `catch (e) { throw new Error(..., { cause: e }) }` of Rust, written inline on the call.
- `ok_or_else` turns a missing value (`Option`) into an error, like `?? throwMissing()`.
<!-- /from -->

<!-- from:default -->
## Key points
- `.map_err(|e| format!("{context}: {e}"))?` adds context and passes the error up in one line.
- `.ok_or_else(|| ...)` turns `None` into an error with context.
- Context goes in front: the outermost layer's words come first.
<!-- /from -->

## Common mistake
Replacing the error instead of extending it:

```rust
raw.parse::<u8>().map_err(|_| "bad value".to_string())  // which field? why?
raw.parse::<u8>().map_err(|e| format!("{field}: {e}"))   // both kept
```

## In this exercise
`numbers::parse_number` returns `Err` messages with the reason. `load_setting` finds the key and wraps any error from it with `map_err`, so the message mentions both the key and the reason.
