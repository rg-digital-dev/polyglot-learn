# Parsing a line, with enums and FromStr

This project builds `logq`, a small tool that reads log files, filters them and sums them up. It's one crate that grows over eight stages, and each stage adds a module. Everything starts with one line of a log:

```text
2026-11-11T08:00:00Z WARN  http: slow request (812 ms)
```

A Rust enum is the natural type for the level: a value that is exactly one of a few cases. Deriving `PartialOrd` and `Ord` orders the cases as they're declared, so `Level::Warn > Level::Info` works with no code of your own.

Turning text into a type is the job of the `FromStr` trait. Once it's implemented, `"warn".parse::<Level>()` calls it:

```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Suit { Hearts, Spades }

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Suit, String> {
        match s.to_ascii_lowercase().as_str() {
            "hearts" => Ok(Suit::Hearts),
            "spades" => Ok(Suit::Spades),
            other => Err(format!("no suit {other:?}")),
        }
    }
}

fn main() {
    assert_eq!("Hearts".parse::<Suit>(), Ok(Suit::Hearts));
    assert!("clubs".parse::<Suit>().is_err());

    // Splitting text never copies it: both halves are slices of `line`.
    let line = "INFO   db: connected";
    let (level, rest) = line.split_once(char::is_whitespace).unwrap();
    assert_eq!((level, rest.trim_start()), ("INFO", "db: connected"));
}
```

`?` returns early with the error. It works on `Option` too, once `.ok_or(…)` has turned the `None` into an error value.

<!-- from:typescript -->
## Coming from TypeScript
- A Rust enum is a closed set like a union of string literals (`"TRACE" | "DEBUG" | …`), but the compiler also checks every `match` covers all of them.
- `FromStr` plus `.parse()` is the Rust version of a `parseLevel(s): Level` function. The difference is that failure is part of the type (`Result`), not an exception you might forget.
- `split_once` returns slices that point into the original string, where JavaScript's `split` builds new strings.
<!-- /from -->

<!-- from:default -->
## Key points
- Derive `PartialOrd, Ord` on an enum to order its cases as declared.
- Implement `FromStr` once, and `.parse::<T>()` works everywhere.
- Errors are values: `ParseError` says what's wrong, and `Display` says it in words.
<!-- /from -->

## Common mistake
Splitting on a single space with `split(' ')`. Logs pad levels to line them up (`WARN  http:`), so that gives empty words between the spaces. Split on whitespace and trim instead.

## In this stage
In `src/entry.rs`, implement `Level::as_str`, `FromStr for Level` (any case), `Display for ParseError`, and `parse_line`. `learn` writes `src/lib.rs` for you, adding a module as each stage arrives, and `src/main.rs` waits until stage 6.
