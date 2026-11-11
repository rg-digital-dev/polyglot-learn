# One error type for the whole program

A program that reads files and parses text can fail in several ways. In Rust the usual answer is one enum with a variant per kind of failure, each carrying what's needed to explain it:

```rust
use std::{fmt, io};

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "no such thing: {name}"),
            AppError::Io(e) => write!(f, "i/o error: {e}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::NotFound(_) => None,
        }
    }
}

// This is what lets `?` turn an io::Error into an AppError.
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> AppError {
        AppError::Io(e)
    }
}

fn read(path: &str) -> Result<String, AppError> {
    Ok(std::fs::read_to_string(path)?)
}

fn main() {
    if let Err(e) = read("missing.txt") {
        eprintln!("{e}");
    }
}
```

`source()` links an error to its cause. Callers can then walk the chain, or `downcast_ref` to look at the cause's own type.

Command-line tools report what went wrong through their exit code as well as their message. 0 means success. The BSD `sysexits.h` names a few common codes: 2 for a usage error (by convention), 65 for bad input data, 66 for an input that can't be opened, and 74 for an i/o error.

<!-- from:typescript -->
## Coming from TypeScript
- Instead of `class ParseError extends Error`, one enum lists every error the program has, and a `match` handles each.
- `?` is like letting an exception propagate, but it's visible at each call and checked: it only compiles when `From` can convert the error.
- `source()` is the `cause` option of `new Error(msg, { cause })`.
<!-- /from -->

<!-- from:default -->
## Key points
- One error enum per program (or per library), with `Display` for people and `Debug` for developers.
- `impl From<X> for Error` makes `?` convert X for you.
- Keep the cause in the variant and return it from `source()`.
<!-- /from -->

## Common mistake
Turning every error into a `String` early (`map_err(|e| e.to_string())`). It's quick, but callers can no longer tell the kinds apart, so the exit code, a retry or a friendly message all become guesswork.

## In this stage
In `src/error.rs`, implement `exit_code`, `Display`, `source`, `From<io::Error>`, `parse_numbered` and `read_all`. Add nothing to `lib.rs`: `learn` already declared the `error` module.
