# Numbers in Rust

Integer types have a fixed size (`i64` here), and `/` and `%` on them truncate toward zero, so the remainder takes the dividend's sign. `div_euclid` and `rem_euclid` give a remainder that is never negative, which for a positive divisor is the same as floor division. Overflow is a bug by default: in a debug build (including `cargo test`) it panics, in a release build it wraps. Methods say what you want instead: `checked_*` returns an `Option`, `wrapping_*` wraps, `saturating_*` stops at the limit, `overflowing_*` returns the wrapped value and a flag.

```rust
fn main() {
    println!("{} {}", -7 / 2, -7 % 2);                    // -3 -1
    println!("{} {}", (-7i64).div_euclid(2), (-1i64).rem_euclid(5)); // -4 4
    println!("{} {}", 2.5f64.round(), 2.5f64.round_ties_even());    // 3 2
    println!("{:?}", i64::MAX.checked_add(1));            // None
    println!("{}", i64::MAX.wrapping_add(1));             // -9223372036854775808
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Integer `/` truncates like `Math.trunc(a / b)`; for floats you need `f64`.
- Overflow doesn't lose precision quietly: it panics in tests, so a "works most of the time" product shows up as a crash.
- `f64::round` rounds halves away from zero: `(-2.5f64).round()` is `-3`, while `Math.round(-2.5)` is `-2`.
<!-- /from -->

<!-- from:default -->
## Key points
- `/` and `%` truncate; `div_euclid` and `rem_euclid` don't go negative.
- Overflow panics in debug builds and wraps in release builds; pick a `checked_`, `wrapping_` or `saturating_` method to make it explicit.
- `f64::round` rounds halves away from zero, `round_ties_even` to even.
- `i64::MIN / -1` panics in every build.
<!-- /from -->

## Common mistake
Relying on the release build's wrapping:

```rust
let big = std::hint::black_box(4_294_967_296i64);
let p = big * big;              // panics under `cargo test`, 0 in `--release`
let q = big.checked_mul(big);   // None in both
```

## In these exercises
`wrap_index` and `split_evenly` need floor semantics, so reach for `rem_euclid` and `div_euclid` (the divisor is always positive). `round_percent` can stay in integers. `checked_multiply` is what `checked_mul` was made for; turn its `None` into an `Err`.
