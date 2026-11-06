# Reading files in Rust

`std::fs::read_to_string(path)` reads a whole file into a `String` and returns a `Result<String, io::Error>`. The `io::Error` knows what went wrong ("The system cannot find the file specified", "permission denied") and its `kind()` (`NotFound`, ...), but not which file, so put the path in front of it when you pass it on. `str::lines()` splits on `\n` and also drops a trailing `\r`, and it doesn't produce an empty last item for a final newline. For big files, `BufReader::new(File::open(path)?).lines()` reads one line at a time.

```rust
use std::fs;

fn first_line(path: &str) -> Result<String, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    Ok(text.lines().next().unwrap_or("").to_string())
}

fn main() {
    match first_line("does-not-exist.txt") {
        Ok(line) => println!("{line}"),
        Err(e) => println!("{e}"), // does-not-exist.txt: The system cannot find the file specified. (os error 2)
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `read_to_string` is `readFileSync(path, "utf8")`, but the error is a returned value, and its message doesn't include the path.
- `lines()` handles `\r\n` for you; `split('\n')` would leave the `\r`.
- The text must be valid UTF-8, or you get an error (`fs::read` gives raw bytes instead).
<!-- /from -->

<!-- from:default -->
## Key points
- `fs::read_to_string(path)?` for small text files.
- Add the path to the error: `io::Error` doesn't carry it.
- `lines()` splits on `\n` and drops a trailing `\r`.
<!-- /from -->

## Common mistake
Unwrapping the read:

```rust
let text = fs::read_to_string(path).unwrap(); // a missing file panics: the tests count that as a failure
```

## In this exercise
`read_lines(path)` reads the file, adds the path to any error, and returns the trimmed lines that aren't blank or `#` comments. The tests pass the full path of files `learn` puts next to them.
