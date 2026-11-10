# Operators and Strings in Rust

`==` and `!=` compare two values of the same type, so there is no `===` and no conversion. The logical operators are `&&`, `||` and `!`, and they stop early. Rust has two text types, and `+` works on one shape only: `String + &str`. Mixing text and numbers never works with `+`. To put values in text, use `format!`, which builds a `String`, or `println!`, which prints. Names in braces are filled in: `{name}`, plus options such as `{:.2}` (two decimals) and `{:>5}` (right-align in 5 columns).

```rust
fn main() {
    let name = "Bob";
    let n = 5;
    println!("{}", name == "Bob");                    // --> true
    let hi = String::from("Hi, ") + name + "!";       // String + &str
    println!("{hi}");                                 // --> Hi, Bob!
    println!("{}", format!("{name} has {n} points")); // --> Bob has 5 points
    println!("{:>5}|{:.2}", n, 3.14159);              // -->     5|3.14
    println!("{} {} {}", 7 / 2, -7 / 2, -7 % 4);      // --> 3 -3 -3
    println!("{}", name.trim().to_uppercase());       // --> BOB
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `"a" + "b"` does not compile: the left side must be a `String`. Use `format!("{a}{b}")`.
- `"Score: " + 5` does not compile. There is no coercion in either direction.
- Integer `/` truncates toward zero: `7 / 2` is `3`. Use `7.0 / 2.0` for `3.5`. `%` keeps the sign of the left side.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- `/` on two integers truncates, and there is no `//`.
- `a < b < c` is a compile error. Write `a < b && b < c`.
- `format!("{x}")` is the f-string, and `{x:?}` is close to `repr`.
<!-- /from -->

<!-- from:default -->
## Key points
- Both operands must have the same type, except `String + &str`.
- `{}` shows the display form and `{:?}` the debug form. Write `{{` and `}}` for literal braces.
- `&&` and `||` need `bool` on both sides. Numbers and strings are not truthy or falsy.
<!-- /from -->

## Common mistake
Using `+` to build text from a `&str` and a number. Use `format!("Score: {n}")` instead.
