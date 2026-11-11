# Zero-copy parsing with lifetimes

`parse_line` copies every field into a new `String`. For a summary such as "how many of each level?", nothing needs to be kept, so those copies are wasted work. A struct can hold `&str` slices of the line instead. A lifetime parameter then tells the compiler how long the struct may live: no longer than the text it points into.

```rust
#[derive(Debug)]
struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

/// `key=value`, both halves slices of `line`: nothing is copied.
fn parse_pair(line: &str) -> Option<Pair<'_>> {
    let (key, value) = line.split_once('=')?;
    Some(Pair { key: key.trim(), value: value.trim() })
}

fn main() {
    let text = String::from("name = logq\nlang = rust\n");
    let pairs: Vec<Pair> = text.lines().filter_map(parse_pair).collect();
    println!("{pairs:?}");
    // drop(text); // error: `text` is still borrowed by `pairs`
}
```

`'_` means "the lifetime of the one reference parameter". It's short for `fn parse_pair<'a>(line: &'a str) -> Option<Pair<'a>>`. When a function takes one reference and returns references, Rust works this out on its own.

The payoff: counting a million lines with `parse_ref` allocates nothing per line. And because the compiler checks the lifetimes, a slice can't outlive its text the way a dangling pointer would in C.

<!-- from:typescript -->
## Coming from TypeScript
- JavaScript strings are immutable and shared, so `slice` may or may not copy, and the engine decides. In Rust, `&str` is always a view into existing text, and the type says so.
- Lifetimes have no TypeScript counterpart. They're compile-time labels for "this borrows from that", and they disappear from the compiled program.
- `to_entry()` is the explicit copy you'd make in TypeScript with `{ ...entry }` before keeping data around.
<!-- /from -->

<!-- from:default -->
## Key points
- Slicing (`trim`, `split_once`, `strip_suffix`) never copies, so a parser built from them borrows naturally.
- A struct with references needs a lifetime: `EntryRef<'a>`.
- Return slices of the input (`Vec<&str>`) when the caller keeps the input alive anyway.
<!-- /from -->

## Common mistake
Reaching for `.to_string()` whenever the borrow checker complains. It makes the error go away by copying, which is exactly what this stage avoids. Usually the fix is to keep the text alive longer: read the file into one `String` first, and parse slices of it.

## In this stage
In `src/borrowed.rs`, implement `to_entry`, `parse_ref` (the same rules and errors as `parse_line`), `count_levels` and `targets_at`. That's logq finished. Try `cargo run -q --bin logq -- --stats --jobs 4 *.log` on some logs of your own.
