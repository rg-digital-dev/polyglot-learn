# Writing your own Iterator

`Iterator` is a trait with one required method: `fn next(&mut self) -> Option<Self::Item>`. Return `Some(value)` while there's more and `None` when you're done. Everything else, from `map`, `filter`, `take` and `zip` to `sum`, `count` and `collect`, is a provided method built on `next`, so a type that implements it gets all of them at once. Iterators are lazy: nothing happens until something calls `next`, so an iterator can describe an endless sequence and the caller decides how much of it to take. `type Item` is an *associated type*: each iterator has exactly one item type. When you don't need a named type, return `impl Iterator<Item = T>` from a chain of adapters instead.

```rust
/// Powers of two, forever.
struct Doubling {
    next: Option<u64>,
}

impl Iterator for Doubling {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.next?; // None: we're done
        self.next = value.checked_mul(2); // None once doubling would overflow
        Some(value)
    }
}

fn odd_squares(limit: u32) -> impl Iterator<Item = u32> {
    (1..limit).filter(|n| n % 2 == 1).map(|n| n * n)
}

fn main() {
    let first: Vec<u64> = Doubling { next: Some(1) }.take(5).collect();
    assert_eq!(first, vec![1, 2, 4, 8, 16]);
    assert_eq!(Doubling { next: Some(1) }.count(), 64); // 2^0 up to 2^63
    assert_eq!(odd_squares(6).collect::<Vec<_>>(), vec![1, 9, 25]);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `next()` returning `Option<T>` is the iterator protocol's `{ value, done }` folded into one value: `Some(v)` or `None`.
- You usually don't need a generator function: a struct that keeps its own state works, and adapters like `map` come with it. Since ES2025, TypeScript's iterator helpers (`.map`, `.take`) do the same.
- `impl Iterator<Item = T>` as a return type is like returning `Iterable<T>`: callers know what comes out, not how it's built.
<!-- /from -->

<!-- from:default -->
## Key points
- One method to write, `next`, and dozens of adapters for free.
- Iterators are lazy. `take(3)` on an endless iterator calls `next` three times.
- An iterator can lend out data it points into: `type Item = &'a str` ties each item to the original text, not to the iterator.
- `impl Iterator<Item = T>` returns an adapter chain without naming its (very long) type.
<!-- /from -->

## Common mistake
Returning an iterator whose closures borrow `self`:

```rust
pub fn cells(&self) -> impl Iterator<Item = (u32, u32)> {
    (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
    // error[E0700]: hidden type for `impl Iterator<Item = (u32, u32)>`
    //               captures lifetime that does not appear in bounds
}
```

The closure keeps a reference to `self`, but the return type doesn't say so. Copy the numbers you need into locals first and use `move` closures, so the iterator owns everything it uses. (Adding `+ '_` to the return type also compiles, but ties the iterator to `self`.)

## In this exercise
`src/lib.rs` has two structs that need a `next`: `Steps`, which counts towards an end in either direction without overflowing, and `Words`, which hands out slices of the text it was given. `Grid::cells` needs no struct at all: return a chain of ranges and adapters.
