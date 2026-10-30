# Lifetimes

A reference must never outlive the value it points to. Most of the time the compiler works out how long a borrow lives on its own. When a function returns a reference, though, it has to know which input the result borrows from, so that callers keep that input alive long enough. A **lifetime parameter** such as `'a` names that link. `fn pick<'a>(x: &'a str, y: &'a str) -> &'a str` means "the result lives no longer than both inputs". Lifetimes don't change how long anything lives; they only describe it so the compiler can check it.

```rust
/// The part of `path` after the last '/', borrowed from `path`.
fn file_name(path: &str) -> &str {           // one input: the lifetime is implied
    match path.rfind('/') {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

/// A struct that holds a reference needs a lifetime parameter too.
struct Line<'a> {
    text: &'a str,
}

impl<'a> Line<'a> {
    /// Words borrowed from the text itself, not from the Line.
    fn words(&self) -> Vec<&'a str> {
        self.text.split_whitespace().collect()
    }
}

fn main() {
    let path = String::from("src/bin/main.rs");
    assert_eq!(file_name(&path), "main.rs");

    let text = String::from("borrow, don't copy");
    let words = Line { text: &text }.words(); // the Line is gone after this line
    assert_eq!(words, ["borrow,", "don't", "copy"]);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A garbage collector keeps anything you still reference alive, so TypeScript never needs lifetimes. Rust has no GC; lifetimes are how it proves a reference is still valid.
- Returning `text.slice(0, 5)` in TypeScript makes a new string. Returning `&text[..5]` in Rust is a view into the original: no copy, and so tied to it.
- `<'a>` sits next to type parameters like `<T>`, but it only exists at compile time.
<!-- /from -->

<!-- from:default -->
## Key points
- One reference input: the output borrows from it, and you don't have to write a lifetime.
- Several reference inputs and a reference output: say which input it comes from with `'a`.
- A struct that holds a reference needs `<'a>`, and can't outlive what it points to.
<!-- /from -->

## Common mistake
Returning a reference to something created inside the function:

```rust
fn shout(text: &str) -> &str {
    let loud = text.to_uppercase();
    &loud
    // error[E0515]: cannot return reference to local variable `loud`
}
fn shout(text: &str) -> String {   // fixed: return the owned String
    text.to_uppercase()
}
```

## In this exercise
Implement `first_word`, `longest` and the two `Excerpt` methods in `src/lib.rs` without copying any text: every `&str` you return is a slice of an input. The tests check that the slices point into the original text, and that `sentences` keeps working after the `Excerpt` is dropped.
