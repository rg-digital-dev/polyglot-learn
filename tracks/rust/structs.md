# Structs in Rust

A struct groups named fields into one type. You build a value with a struct literal that names every field, read fields with dot syntax, and attach behaviour in an `impl` block. Methods take `&self` to read the value or `&mut self` to change it. Traits such as `Debug`, `Clone` and `PartialEq` are not there by default; `#[derive(...)]` generates them, which is what lets `assert_eq!` compare and print two structs.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reading {
    station: String,
    celsius: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    low: i64,
    high: i64,
}

impl Range {
    fn spread(&self) -> i64 {
        self.high - self.low
    }
}

fn range_of(readings: &[Reading]) -> Range {
    let mut range = Range { low: i64::MAX, high: i64::MIN };
    for r in readings {
        range.low = range.low.min(r.celsius);
        range.high = range.high.max(r.celsius);
    }
    range
}

fn main() {
    let north = Reading { station: "north".to_string(), celsius: 4 };
    let day = [north.clone(), Reading { celsius: 11, ..north }];
    let range = range_of(&day);
    assert_eq!(range, Range { low: 4, high: 11 });
    assert_eq!(range.spread(), 7);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A struct is closer to a TypeScript `class` with fixed fields than to an `interface`: the type is nominal, so two structs with the same fields are still different types.
- Every field must be given when you build a value; `Reading { celsius: 11, ..north }` fills the rest from another value, a little like spread.
- If a local variable has the same name as a field, `Range { low, high }` is shorthand, much like `{ low, high }`.
<!-- /from -->

<!-- from:default -->
## Key points
- `#[derive(Debug, Clone, PartialEq, Eq)]` gives printing, copying and comparison for free.
- Iterating over `&[T]` yields `&T`; reading `item.field` through the reference is fine.
- `i64` division truncates toward zero, which is the same as rounding down for non-negative values.
<!-- /from -->

## Common mistake
Leaving out a field when building the result:

```rust
let r = Range { low: 1 };            // error[E0063]: missing field `high` in initializer of `Range`
let r = Range { low: 1, high: 9 };   // fixed: every field is listed
```

## In this exercise
Implement `pub fn order_total(items: &[LineItem], discount_percent: i64) -> OrderSummary`. Both structs are generated in your stub. Compute the discount as `subtotal_cents * discount_percent / 100`; dividing first loses cents.
