# Types and Annotations in Rust

Rust is statically typed: the compiler checks every type before the program runs. An annotation follows the name after a colon: `let n: i64 = 7;`. You can often leave it out, because the compiler infers the type from the value. A whole number defaults to `i32` and a decimal to `f64`. There are many integer types (`i8` to `i128`, `u8` to `u128`, `usize`). Rust never converts between them for you: you write `as` or a method such as `i64::from`. There is no `null`: a value that may be missing is an `Option<T>`, either `Some(value)` or `None`.

```rust
fn main() {
    let a = 7;                  // i32 by default
    let b: i64 = 7;
    let d = 2.5;                // f64 by default
    let big = b + a as i64;     // convert explicitly with as
    let half = a as f64 / 2.0;
    println!("{big} {half} {d}"); // --> 14 3.5 2.5
    let n: i32 = "42".parse().unwrap();
    println!("{}", n + 1);        // --> 43
    let maybe: Option<i32> = None;
    println!("{maybe:?} {:?}", Some(3)); // --> None Some(3)
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Types are real, not erased: they decide how the value is stored. There is no `number`; pick `i64`, `f64` and so on.
- `1 == 1.0` and `i64 + i32` do not compile. Convert with `as f64` or `i64::from(x)`.
- `Option<T>` replaces `null` and `undefined`. You cannot use the value until you handle `None`.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Annotations are enforced: the compiler refuses a program that breaks them.
- Integers have a fixed size. `u8` holds 0 to 255, and `300_i32 as u8` gives `44` because `as` wraps. Python's `int` never overflows.
- Dividing two integers gives an integer: `7 / 2` is `3`.
<!-- /from -->

<!-- from:default -->
## Key points
- The compiler checks all types before anything runs. Type errors come first; other problems show once those are fixed.
- Defaults are `i32` and `f64`. The course's stubs mostly use `i64`, so match the signature.
- `&str` is borrowed text and `String` is owned text. Take `&str` as input and return `String` when you build text.
- Parsing text gives a `Result` (`Ok(value)` or `Err(...)`). `.unwrap()` takes the value or panics.
<!-- /from -->

## Common mistake
Mixing integer sizes, such as adding an `i64` parameter to a `.len()` result (`usize`). Convert one side: `xs.len() as i64`.
