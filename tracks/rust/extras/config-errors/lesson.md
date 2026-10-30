# Option, Result and your own error types

Rust uses two enums where other languages use `null` and exceptions. `Option<T>` says a value may be missing. `Result<T, E>` says an operation may fail with an error of type `E`. Defining `E` as your own enum, with one variant per way things can go wrong, lets callers `match` on the reason, and implementing `Display` gives each variant a readable message. The `?` operator returns early with the error (converting it when there's a `From` implementation, as there is for `Box<dyn Error>`). Combinators like `ok_or`, `map_err` and `unwrap_or` convert between Option, Result and plain values.

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum AgeError {
    Empty,
    NotANumber(String),
    TooOld(u32),
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgeError::Empty => write!(f, "no age given"),
            AgeError::NotANumber(s) => write!(f, "`{s}` is not a number"),
            AgeError::TooOld(n) => write!(f, "{n} is too old"),
        }
    }
}

impl std::error::Error for AgeError {}

fn parse_age(input: &str) -> Result<u32, AgeError> {
    let text = Some(input.trim()).filter(|t| !t.is_empty()).ok_or(AgeError::Empty)?;
    let age: u32 = text.parse().map_err(|_| AgeError::NotANumber(text.to_string()))?;
    if age > 150 {
        return Err(AgeError::TooOld(age));
    }
    Ok(age)
}

fn main() {
    assert_eq!(parse_age(" 42 "), Ok(42));
    assert_eq!(parse_age("old"), Err(AgeError::NotANumber("old".into())));
    assert_eq!(parse_age("").unwrap_err().to_string(), "no age given");
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `Option<T>` is `T | undefined`, but you must handle the missing case before you can use the value.
- Errors are return values, not exceptions: `Result<T, E>` appears in the signature, so callers see that a function can fail.
- `?` is like letting an exception propagate, but explicit: one character at each place that can fail.
<!-- /from -->

<!-- from:default -->
## Key points
- `ok_or(err)` turns an Option into a Result; `.ok()` goes the other way.
- `map_err` replaces an error you can't use (like `ParseIntError`) with your own.
- Implement `Display` and `std::error::Error` so your error works with `?` in functions returning `Box<dyn Error>`.
<!-- /from -->

## Common mistake
Using `?` on an Option inside a function that returns a Result:

```rust
let (k, v) = line.split_once('=')?;
// error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s,
//               in a function that returns `Result`
let (k, v) = line.split_once('=').ok_or(ConfigError::BadLine(n))?;  // fixed
```

## In this exercise
`src/lib.rs` defines `Config` and `ConfigError`. Implement `Display` for the error, then `parse_pairs`, `get` (an Option) and `parse_config`, which passes every failure up with `?`. The last test uses your error through a `Box<dyn Error>`.
