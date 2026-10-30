# Enums and match

A Rust `enum` lists the forms a value can take, and each variant can carry its own data: none, a tuple of values, or named fields. `match` takes a value apart variant by variant and binds the data inside to names. The compiler checks that a `match` covers every variant, so adding a variant later points you at every place that needs to handle it. `Option<T>` (`Some(value)` or `None`) is an ordinary enum from the standard library, used wherever there may be no value.

```rust
#[derive(Debug, PartialEq)]
enum Shape {
    Circle(f64),                         // tuple variant
    Rect { width: f64, height: f64 },    // struct variant
    Dot,                                 // no data
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect { width, height } => width * height,
        Shape::Dot => 0.0,
    }
}

fn parse(text: &str) -> Option<Shape> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    match parts.as_slice() {
        ["dot"] => Some(Shape::Dot),
        ["circle", r] => r.parse().ok().map(Shape::Circle),
        ["rect", w, h] => Some(Shape::Rect { width: w.parse().ok()?, height: h.parse().ok()? }),
        _ => None,
    }
}

fn main() {
    assert_eq!(area(&Shape::Rect { width: 2.0, height: 3.0 }), 6.0);
    assert_eq!(parse("rect 2 3"), Some(Shape::Rect { width: 2.0, height: 3.0 }));
    assert_eq!(parse("hexagon"), None);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- An enum with data is Rust's version of a discriminated union like `{ kind: "circle"; r: number } | { kind: "dot" }`, with the tag built in.
- `match` is a `switch` that must be exhaustive and that pulls the fields out for you. A missing case is a compile error, not a runtime surprise.
- `Option<T>` replaces `T | undefined`. You can't use the value without first handling `None`.
<!-- /from -->

<!-- from:default -->
## Key points
- Variants can hold nothing, unnamed values `Forward(i32)`, or named fields `Teleport { x, y }`.
- `match` must cover every variant; `_` matches anything that's left.
- `if let Some(x) = value { ... }` handles one variant and ignores the rest.
<!-- /from -->

## Common mistake
Forgetting a variant:

```rust
match cmd {
    Command::Forward(n) => { /* ... */ }
    Command::Say(text) => { /* ... */ }
}
// error[E0004]: non-exhaustive patterns: `&Command::Turn(_)` and `&Command::Teleport { .. }` not covered
```

Add the missing arms. Reach for `_ => {}` only when ignoring the other variants is really what you mean.

## In this exercise
`src/lib.rs` already defines `Direction`, `Turn`, `Command` and `Robot`. Implement `Direction::turn`, `parse` (text to `Option<Command>`), `Robot::new`, `Robot::apply` (one `match` over the command) and `run`, which skips lines that don't parse.
