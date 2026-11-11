# Streaming input with BufRead

A log can be bigger than memory. Reading it a line at a time keeps memory use flat however large the file is. `BufRead` is the trait for readers that can do that, and `lines()` turns one into an iterator:

```rust
use std::io::{self, BufRead, Cursor};

/// Lengths of the non-empty lines, read lazily.
fn lengths<R: BufRead>(reader: R) -> impl Iterator<Item = io::Result<usize>> {
    reader
        .lines()
        .filter_map(|line| match line {
            Ok(text) if text.is_empty() => None,
            Ok(text) => Some(Ok(text.len())),
            Err(e) => Some(Err(e)),
        })
}

fn main() {
    // Any BufRead works: a Cursor over a string here, a file or stdin in logq.
    let input = Cursor::new("one\n\nthree\n");
    for n in lengths(input) {
        println!("{}", n.unwrap());
    }
    let _stdin = io::stdin().lock(); // also a BufRead
}
```

`impl Iterator<Item = …>` in a return type means "some iterator; the caller doesn't need its exact type". The chain does nothing until someone calls `next()`, so it stays lazy all the way.

The same function reads a file (`BufReader::new(File::open(path)?)`), standard input (`io::stdin().lock()`) or a string in a test (`Cursor::new(text)`), because it asks only for `BufRead`.

<!-- from:typescript -->
## Coming from TypeScript
- `reader.lines()` is Node's `readline` interface or `for await (const line of rl)`, but synchronous, with no events.
- Iterator adapters are lazy like a generator function: `.filter_map(...)` runs nothing until the loop asks for the next item.
- Generics such as `R: BufRead` are checked at compile time and cost nothing at run time, a bit like an interface parameter that's resolved before the program runs.
<!-- /from -->

<!-- from:default -->
## Key points
- Take `impl BufRead` (or `R: BufRead`), not a file name, so tests can pass a `Cursor`.
- Return `impl Iterator` to stay lazy: no `Vec` of every line.
- A bad line is an `Err` item; the caller decides whether to stop.
<!-- /from -->

## Common mistake
`read_to_string` followed by `.lines()`. It works, but it holds the whole log in memory, and nothing comes out until the end of the file. On a log that's still being written, the end never comes.

## In this stage
In `src/input.rs`, implement `entries` (count lines from 1, blank lines included) and `open`.
