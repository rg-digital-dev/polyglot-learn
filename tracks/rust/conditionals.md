# Conditionals in Rust

`if` in Rust takes a condition of type `bool`, written without parentheses, and the braces are always required. `if` is an expression, so `let x = if cond { a } else { b };` works and takes the place of a ternary operator. For several cases, `match` compares a value against patterns and must cover every possibility. Boolean logic uses `&&`, `||` and `!`, which short-circuit.

```rust
fn shipping_cost(weight_kg: i64, express: bool) -> i64 {
    let base = if weight_kg <= 2 {
        5
    } else if weight_kg <= 10 {
        9
    } else {
        20
    };
    match express {
        true => base * 2,
        false => base,
    }
}

fn is_weekend(day: u8) -> bool {
    day == 6 || day == 7
}
```

<!-- from:typescript -->
## Coming from TypeScript
- No truthiness: `if count { }` does not compile. Write `if count != 0 { }`.
- No `? :` ternary. Use `if a { x } else { y }` as an expression; both branches must have the same type.
- `==` always compares values of the same type, so there is no `===`.
- Precedence of `&&` over `||` is the same as in TS, but parentheses still help readers.
- `%` keeps the sign of the left operand, like JS: `-7 % 4` is `-3`.
<!-- /from -->

<!-- from:default -->
## Key points
- Conditions must be `bool`; numbers and strings are never implicitly converted.
- `if`/`else` is an expression with a value, so branches can be assigned directly.
- A function that returns `bool` can often be a single expression combining `&&` and `||`.
<!-- /from -->

## Common mistake
Using a number as a condition:

```rust
if year % 4 { /* ... */ }        // error: expected `bool`, found `i64`
if year % 4 == 0 { /* ... */ }   // fixed
```

## In this exercise
Implement `pub fn leap_year(year: i64) -> bool`. The whole body can be one boolean expression; no `if` is strictly needed.
