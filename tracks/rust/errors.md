# Errors in Rust

Rust has no exceptions for expected failures. A function that can fail returns `Result<T, E>`, which is either `Ok(value)` or `Err(error)`, and the caller has to deal with both. The `?` operator returns early with the error if there is one, and `map_err` converts one error type into another. `panic!` (and `unwrap`, `expect`, `todo!`) is meant for bugs, not for bad input.

```rust
fn parse_percent(input: &str) -> Result<u8, String> {
    let digits = input.trim().trim_end_matches('%');
    let n: u32 = digits
        .parse()
        .map_err(|_| format!("not a number: {digits:?}"))?;
    if n > 100 {
        return Err(format!("{n} is above 100"));
    }
    Ok(n as u8)
}

fn main() {
    assert_eq!(parse_percent(" 42% "), Ok(42));
    assert!(parse_percent("abc").is_err());
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Instead of `throw`/`try`/`catch`, failures are ordinary return values of type `Result`.
- The compiler warns if you ignore a `Result`, so errors are hard to forget.
- `?` is roughly "rethrow": it hands the `Err` to your caller immediately.
- `str::parse` is stricter than `parseInt`: `"80a"` is an error, not `80`. It still accepts a leading `+`.
- Integer types have fixed sizes; parsing a huge number returns an `Err` instead of losing precision.
<!-- /from -->

<!-- from:default -->
## Key points
- Return `Err(...)` for invalid input; keep `panic!` for programming mistakes.
- `?` propagates errors; `map_err` turns a library error into your own type or message.
- `str::parse::<T>()` checks the range of `T` for you, and it accepts a leading `+`.
<!-- /from -->

## Common mistake
Calling `.unwrap()` on a parse result:

```rust
let n: i64 = input.trim().parse().unwrap(); // panics on "80a"
let n: i64 = input.trim().parse().map_err(|_| "not a number".to_string())?; // returns Err
```

## In this exercise
Implement `pub fn parse_port(input: &str) -> Result<i64, String>`. Invalid input must return `Err(...)` with a message; the tests only check that it is an error, and a panic fails them.
