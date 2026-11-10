# Blocks and Control Flow in Rust

Blocks use braces. Conditions have no parentheses, but the braces are always required. `if`, `match` and `loop` are expressions: they produce a value, so `let size = if n > 5 { "big" } else { "small" };` takes the place of a ternary. Both branches must have the same type, and an `if` used as a value needs an `else`. `for x in &xs` visits items, `for i in 0..10` counts up to 9 (`0..=10` includes 10), `while` repeats on a condition, and `loop` repeats until `break`, which can carry a value. `match` picks by pattern, and the compiler requires every case to be covered.

```rust
fn describe(n: i32) -> &'static str {
    match n {
        3..=9 => "medium",
        _ => "large", // _ covers everything else
    }
}

fn main() {
    let n = 7;
    let size = if n > 5 { "big" } else { "small" };
    println!("{size} {}", describe(n)); // --> big medium
    let mut i = 0;
    let found = loop {
        i += 1;
        if i * 10 > 25 { break i * 10; }
    };
    println!("{found}"); // --> 30
}
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no fall-through and no `break` in `match`. A case runs alone.
- `for (const x of xs)` becomes `for x in &xs`, and `for i in 0..n` replaces the counting loop.
- `if x { }` needs a `bool`. `0` and `""` are not conditions.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- `range(10)` is `0..10`. Both stop before 10.
- `elif` is `else if`. There is no `while ... else`.
<!-- /from -->

<!-- from:default -->
## Key points
- Conditions must be `bool`. `if`, `match` and `loop` give back values.
- A `match` that misses a case does not compile (E0004). Use `_` as the catch-all.
- `break` and `continue` work as usual, and `break value` leaves a `loop` with a value.
- An `if` used as a value without `else`, or with branches of different types, does not compile (E0317, E0308).
<!-- /from -->
