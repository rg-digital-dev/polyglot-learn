# Output formats with traits, and a testable `run`

logq prints in two formats. A trait names what every format can do, and `Box<dyn Format>` holds whichever one the command line picked. Which method runs is decided at run time:

```rust
use std::io::{self, Write};

trait Shout {
    fn say(&self, out: &mut dyn Write, text: &str) -> io::Result<()>;
}

struct Loud;
struct Quiet;

impl Shout for Loud {
    fn say(&self, out: &mut dyn Write, text: &str) -> io::Result<()> {
        writeln!(out, "{}!", text.to_uppercase())
    }
}

impl Shout for Quiet {
    fn say(&self, out: &mut dyn Write, text: &str) -> io::Result<()> {
        writeln!(out, "({text})")
    }
}

fn pick(loud: bool) -> Box<dyn Shout> {
    if loud { Box::new(Loud) } else { Box::new(Quiet) }
}

fn main() -> io::Result<()> {
    let mut buffer = Vec::new(); // a Vec<u8> is a Write too: handy in tests
    pick(true).say(&mut buffer, "hi")?;
    pick(false).say(&mut io::stdout(), "hi")?;
    assert_eq!(buffer, b"HI!\n");
    Ok(())
}
```

The same idea makes the whole program testable. If `run` takes its arguments, input, output and error streams as parameters and returns the exit code, a test can call it with a `Cursor` and two `Vec<u8>`s. `main` then shrinks to passing in the real ones and calling `std::process::exit`.

A tool's real output goes to stdout and its complaints to stderr. That way `logq app.log > errors.txt` still shows warnings on screen and keeps them out of the file.

<!-- from:typescript -->
## Coming from TypeScript
- A trait is an interface; `Box<dyn Format>` is a value typed by its interface, like `const f: Format = json ? new Json() : new Text()`.
- `&mut dyn Write` is the Rust `Writable` stream: stdout, a file, or a `Vec<u8>` in memory.
- `std::process::exit(code)` is `process.exit(code)`. Like it, it skips destructors, so call it last.
<!-- /from -->

<!-- from:default -->
## Key points
- A trait plus `Box<dyn Trait>` picks behaviour at run time; generics pick it at compile time.
- Write to `&mut dyn Write`, never straight to `println!`, so tests can capture the output.
- Keep `main` tiny: parse nothing there, just call `run` and exit with its code.
<!-- /from -->

## Common mistake
Building JSON with `format!("\"{}\"", text)`. One quote or newline in a log message breaks every program reading the output. Escape it with `json_string`, or use a library (serde_json) in real code.

## In this stage
Implement `src/output.rs` (`formatter`, `json_string`, `Text`, `Json`) and `run` in `src/run.rs`, then replace `main` in `src/main.rs` so it calls `log_cli::run::run`. Then try it:

```sh
cargo run -q --bin logq -- --level warn some.log
```
