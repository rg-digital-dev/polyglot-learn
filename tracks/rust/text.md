# Text in Rust

A `String` (and a `&str`) is UTF-8 bytes that Rust guarantees are valid. `s.len()` counts bytes. Indexing with `s[i]` doesn't compile, and slicing with `&s[a..b]` uses byte offsets and **panics** if an offset falls inside a character. A `char` is one code point (a Unicode scalar value): `s.chars()` iterates them, and `s.char_indices()` gives each one with its byte offset, which is what you need to slice safely.

```rust
fn main() {
    let s = "Zoë 😀";
    println!("{}", s.len());               // 9 bytes
    println!("{}", s.chars().count());     // 5 chars
    let (i, _) = s.char_indices().nth(3).unwrap();
    println!("{:?}", &s[..i]);             // "Zoë": cut at a char boundary
    // &s[..3] would panic: byte 3 is inside 'ë'
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `s.len()` is the UTF-8 byte length, not `s.length` (UTF-16) and not the number of characters.
- `s.chars()` is `[...s]`: one `char` per code point, emoji included.
- There's no `Intl.Segmenter` in `std`; the `unicode-segmentation` crate adds graphemes.
<!-- /from -->

<!-- from:default -->
## Key points
- `len()` and slice ranges are in bytes; slicing inside a character panics.
- `chars()` and `char_indices()` walk code points.
- `s.is_char_boundary(i)` tells you whether a byte offset is safe to cut at.
<!-- /from -->

## Common mistake
Using the character count as a byte offset:

```rust
let s = "résumé";
let n = 2;
// &s[..n]                                  // panics: byte 2 is inside 'é'
let head: String = s.chars().take(n).collect();   // "ré"
```

## In these exercises
`char_count(text)` returns a `HashMap<String, i64>`; `ch.to_string()` turns a `char` into a key. `truncate(text, limit)` counts with `chars().count()` and builds the result from `chars().take(..)`. `limit` is an `i64`, so convert it with `as usize` once you know it isn't negative. The *Deeper Rust* exercise `text-slices` goes further into `&str` borrowing.
