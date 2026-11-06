# Cow and AsRef: borrow when you can, own when you must

Many functions that clean up text change nothing most of the time. Returning a `String` means allocating a copy anyway. `Cow<'a, str>` ("clone on write") is an enum with two variants: `Cow::Borrowed(&'a str)`, a slice of the input, and `Cow::Owned(String)`, a new value. The function decides at run time, so the common case costs nothing and only the inputs that change pay for a new String. A `Cow<str>` dereferences to `&str`, so callers use it like any string, and `into_owned()` turns it into a `String` when they need to keep it. On the parameter side, `AsRef<str>` is the matching tool: a generic `S: AsRef<str>` accepts `&str`, `String` and anything else that can lend out a `&str`.

```rust
use std::borrow::Cow;

/// Spaces become underscores, if there are any.
fn snake(name: &str) -> Cow<'_, str> {
    if name.contains(' ') {
        Cow::Owned(name.replace(' ', "_"))
    } else {
        Cow::Borrowed(name)
    }
}

fn longest<S: AsRef<str>>(items: &[S]) -> usize {
    items.iter().map(|s| s.as_ref().len()).max().unwrap_or(0)
}

fn main() {
    assert!(matches!(snake("user_id"), Cow::Borrowed(_)));
    assert_eq!(snake("user id"), "user_id");
    let kept: String = snake("first name").into_owned();
    assert_eq!(kept, "first_name");

    assert_eq!(longest(&["a", "abc"]), 3);
    assert_eq!(longest(&[String::from("ab")]), 2);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- In TypeScript every string is shared and immutable, so returning the input unchanged is always free. In Rust, a `String` return forces a copy; `Cow` gets the "return the input if nothing changed" trick back.
- `S: AsRef<str>` is the closest thing to accepting `string | String` without overloads.
- The lifetime in `Cow<'a, str>` says the borrowed case points into the caller's input, so the result can't outlive it.
<!-- /from -->

<!-- from:default -->
## Key points
- `Cow::Borrowed(slice)` when nothing changed, `Cow::Owned(string)` when something did. Check first, then build.
- A sub-slice such as `text.trim()` or `&text[2..]` is still borrowed: no copy needed.
- `Cow<str>` works like `&str` (it implements `Deref`), compares with `==` and turns into a `String` with `into_owned()`.
- `AsRef<str>` on a parameter accepts `&str` and `String`. For one value, `impl AsRef<str>` is shorter.
<!-- /from -->

## Common mistake
Building the new String first and only then checking whether it differs:

```rust
let fixed = text.replace('<', "&lt;");
if fixed == text { Cow::Borrowed(text) } else { Cow::Owned(fixed) }
```

It returns the right variant, but it has already allocated: the saving `Cow` is there for is gone. Test the input (with `contains`, `any` or `find`), and allocate only on the branch that needs it.

## In this exercise
`src/lib.rs` has three functions that return `Cow<str>`. The tests check the result's value and also whether it is a slice of the input. `total_len` takes a generic `S: AsRef<str>`, so the same function counts a list of `&str` and a list of `String`.
