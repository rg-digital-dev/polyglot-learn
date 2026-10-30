# Generics and trait bounds

A generic type or function takes a type parameter, `<T>`, so one definition works for many types. Inside, Rust only lets you do with a `T` what you've declared it can do. A **trait bound** such as `T: PartialOrd` ("T can be compared") or `T: Display` ("T can be printed") adds those abilities, and callers can only use types that have them. Bounds can sit on a function, or on an `impl` block, so that some methods only exist for some `T`. The compiler makes a separate copy of the code for each type you use (monomorphisation), so generics cost nothing at run time.

```rust
use std::fmt::Display;

/// A pair of values of the same type.
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn swap(self) -> Pair<T> {
        Pair { a: self.b, b: self.a }
    }
}

// Only pairs of comparable, printable values get `bigger`.
impl<T: PartialOrd + Display> Pair<T> {
    fn bigger(&self) -> String {
        if self.a >= self.b { format!("{}", self.a) } else { format!("{}", self.b) }
    }
}

fn smallest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    items.iter().copied().fold(None, |min, x| match min {
        Some(m) if m <= x => Some(m),
        _ => Some(x),
    })
}

fn main() {
    let p = Pair { a: 3, b: 8 }.swap();
    assert_eq!((p.a, p.b), (8, 3));
    assert_eq!(Pair { a: "pear", b: "fig" }.bigger(), "pear");
    assert_eq!(smallest(&[4.5, 1.25, 9.0]), Some(1.25));
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `<T>` reads the same, but a bare `T` can do almost nothing in Rust: no comparing, printing or copying until a bound allows it.
- `T: PartialOrd` plays the role of `T extends Comparable`, except that types opt in by implementing the trait.
- TypeScript erases generics; Rust generates specialised code for each type you use.
<!-- /from -->

<!-- from:default -->
## Key points
- Put bounds where they're needed: on the function, or on an `impl<T: Bound>` block for some methods only.
- `Copy` lets you return a `T` out of a borrowed slice; without it, return `&T` instead.
- Several bounds combine with `+`; long lists read better in a `where` clause.
<!-- /from -->

## Common mistake
Comparing values of a type with no bound:

```rust
fn largest<T>(items: &[T]) -> Option<&T> {
    // ...
    if item > best { /* ... */ }
    // error[E0369]: binary operation `>` cannot be applied to type `&T`
}
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {  // fixed: add the bound
```

## In this exercise
Implement `Stack<T>` in `src/lib.rs`: the basic methods work for any `T`, and `describe` only exists when `T: Display`. Then write `largest` (needs `PartialOrd + Copy`) and `largest_by_key`, which returns a reference so it needs no `Copy`.
