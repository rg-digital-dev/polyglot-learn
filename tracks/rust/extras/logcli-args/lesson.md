# Parsing arguments by hand

Most Rust tools use the `clap` crate for their command line, but parsing by hand is a good way to see iterators and pattern matching at work, and it needs nothing outside the standard library. `std::env::args()` is an iterator of `String`s; the first is the program's name.

The trick is that the loop and the option both pull from the same iterator. An option such as `--level` takes the next argument as its value:

```rust
#[derive(Debug, Default)]
struct Options {
    name: Option<String>,
    verbose: bool,
    files: Vec<String>,
}

fn parse(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut out = Options::default();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--name" => out.name = Some(args.next().ok_or("--name needs a value")?),
            "-v" => out.verbose = true,
            _ if arg.starts_with('-') => return Err(format!("unknown option {arg}")),
            _ => out.files.push(arg),
        }
    }
    Ok(out)
}

fn main() {
    let input = ["--name", "Ada", "-v", "notes.txt"].map(String::from);
    println!("{:?}", parse(input)); // Ok(Options { name: Some("Ada"), verbose: true, files: ["notes.txt"] })
    println!("{:?}", parse(std::env::args().skip(1))); // the real command line
}
```

`while let Some(x) = it.next()` is the loop to use when the body also calls `next()`. A `for` loop holds on to the iterator, so the body can't touch it.

By convention, `--name=value` works as well as `--name value`, a lone `-` means standard input, and `--` ends the options, so a file named `-x.log` can still be read.

<!-- from:typescript -->
## Coming from TypeScript
- `std::env::args()` is `process.argv`, except it starts at the program's name rather than at `node`. Skip one, not two.
- A `match` on `arg.as_str()` replaces a `switch` on strings. Guards (`_ if arg.starts_with('-')`) cover what `case` can't.
- There's no `undefined` for a missing value: `args.next()` is an `Option`, and you have to decide what `None` means.
<!-- /from -->

<!-- from:default -->
## Key points
- Use `while let Some(arg) = args.next()` so an option can take its value with `args.next()`.
- Fill in the defaults first, then let each option change one field.
- A usage error should name the option or value, and show the usage line.
<!-- /from -->

## Common mistake
Treating every argument that starts with `-` as an option. `-` on its own is a file (standard input), and so is everything after `--`.

## In this stage
Implement `Args::parse` in `src/args.rs`. The doc comment lists the options; usage errors end with a line holding `USAGE`.
