# From, Into and how ? converts errors

`?` does more than return early. On an `Err(e)` it returns `Err(From::from(e))`, so the error is converted into the function's own error type, as long as there's a `From` implementation for it. Write `impl From<ParseIntError> for AppError` once, and every `?` on a number parse inside a function returning `Result<_, AppError>` converts by itself, with no `map_err`. Implementing `From<A> for B` also gives you `Into<B> for A` for free, so `let b: B = a.into();` works too. Two more pieces make a well-behaved error: `Display` for the message a person reads, and `Error::source()` to point at the lower-level error it wraps, so a caller can walk the chain. The standard library has `impl<E: Error> From<E> for Box<dyn Error>`, which is why `?` also turns any error into a `Box<dyn Error>`.

```rust
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
enum PriceError {
    Bad(ParseFloatError),
}

impl From<ParseFloatError> for PriceError {
    fn from(e: ParseFloatError) -> PriceError {
        PriceError::Bad(e)
    }
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceError::Bad(_) => write!(f, "not a price"),
        }
    }
}

impl Error for PriceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PriceError::Bad(e) => Some(e),
        }
    }
}

fn price_in_cents(text: &str) -> Result<u64, PriceError> {
    let euros: f64 = text.trim_start_matches('€').parse()?; // ParseFloatError -> PriceError
    Ok((euros * 100.0).round() as u64)
}

fn main() {
    assert_eq!(price_in_cents("€2.50").unwrap(), 250);
    let err = price_in_cents("two").unwrap_err();
    assert_eq!(err.to_string(), "not a price");
    assert!(err.source().is_some()); // the ParseFloatError underneath
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `source()` is the `cause` in `new Error("not a price", { cause: err })`: the error underneath this one.
- `From` replaces a `catch` block that wraps one error type in another. You write the conversion once, and every `?` uses it.
- `downcast_ref::<AppError>()` on a `Box<dyn Error>` is like `err instanceof AppError`, but it gives you the typed value too.
<!-- /from -->

<!-- from:default -->
## Key points
- `?` on `Err(e)` returns `Err(From::from(e))`. No `From` implementation, no conversion: the compiler says "`?` couldn't convert the error".
- `impl From<A> for B` gives you `a.into()` as well. Implement `From`, never `Into`.
- `source()` returns the wrapped error, so callers can follow the chain. Return `None` for errors that are the root cause.
- `Box<dyn Error>` accepts any error through `?`. `downcast_ref::<T>()` gets the concrete type back.
<!-- /from -->

## Common mistake
Putting the wrapped error's message into your own message *and* returning it from `source()`. A tool that prints the whole chain then says it twice:

```rust
// Error: not a number: invalid digit found in string
// Caused by: invalid digit found in string
```

Pick one. This exercise uses both on purpose, so you see each in action; in real code, many libraries leave the cause out of `Display`.

## In this exercise
`src/lib.rs` defines `AppError`. Implement `From<ParseIntError>`, `Display` and `source()`, then `parse_field` (one `?`, no `map_err`), `parse_rgb` and `run`, which hands every error back in a `Box<dyn Error>`.
