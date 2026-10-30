# Loops in Rust

Rust has three loops: `for x in iterable`, `while cond`, and `loop` (runs until `break`). `for` is the everyday choice: it walks a range such as `1..10` (end excluded) or `1..=10` (end included), or anything else that can be iterated, like a slice. `continue` skips to the next pass and `break` leaves the loop. Many loops can also be written with iterator methods such as `filter`, `any` and `sum`.

```rust
fn sum_of_even_squares(limit: i64) -> i64 {
    let mut total = 0;
    for n in 1..limit {
        if n % 2 != 0 {
            continue;
        }
        total += n * n;
    }
    total
}

fn contains_negative(values: &[i64]) -> bool {
    // iterator version: stops at the first match
    values.iter().any(|&v| v < 0)
}
```

`|&v| v < 0` is a closure: a small unnamed function, here taking each value and returning whether it's negative. Closures get their own lesson (9, Make Counter); for now, read `|x| expr` as "given x, compute expr".

<!-- from:typescript -->
## Coming from TypeScript
- `for n in 1..limit` replaces `for (let i = 1; i < limit; i++)`; there is no C-style `for`.
- `for x in slice` is like `for...of`, but each `x` is a reference (`&i64`), not a copy.
- Variables are immutable by default: a running total needs `let mut`.
- `.iter().any(...)`, `.filter(...)` and `.sum()` resemble `some`, `filter` and `reduce`, but do nothing until consumed.
- `n % 0` panics at runtime instead of producing `NaN`.
<!-- /from -->

<!-- from:default -->
## Key points
- `a..b` excludes `b`; `a..=b` includes it.
- Iterating a slice yields references; use the pattern `for &x in values` or write `*x` to get the number.
- Accumulators must be declared with `let mut`.
<!-- /from -->

## Common mistake
Comparing a reference with a number:

```rust
for f in factors {
    if f == 0 { continue; }   // error: can't compare `&i64` with `{integer}`
}
for &f in factors {
    if f == 0 { continue; }   // fixed: the pattern copies the value out
}
```

## In this exercise
Implement `pub fn sum_of_multiples(factors: &[i64], limit: i64) -> i64`. Skip a factor of 0 before using `%`, or the test run will panic.
