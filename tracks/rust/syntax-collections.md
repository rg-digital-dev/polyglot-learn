# Lists, Maps and Tuples in Rust

Rust's growable list is `Vec<T>`, made with `vec![1, 2, 3]` and counted from 0. A fixed-size `[i32; 3]` is an array. A map is a `HashMap<K, V>`, imported with `use std::collections::HashMap;`. A tuple is `("alice", 3)`, read with `t.0` and `t.1` or unpacked with `let (name, score) = t;`. Indexing with `xs[10]` panics when out of range, so use `xs.get(10)`, which returns an `Option`. Reading a map with `m.get("k")` also gives an `Option`: `Some(value)` or `None`. A collection must be `let mut` to change.

```rust
use std::collections::HashMap;

fn main() {
    let mut xs = vec![1, 2, 3];
    xs.push(4);
    println!("{} {} {:?}", xs[0], xs.len(), xs);          // --> 1 4 [1, 2, 3, 4]
    println!("{:?} {:?}", xs.get(1), xs.get(10));         // --> Some(2) None
    let mut m = HashMap::new();
    m.insert("alice", 3);
    println!("{:?} {:?}", m.get("alice"), m.get("zed"));  // --> Some(3) None
    *m.entry("alice").or_insert(0) += 1;
    println!("{}", m["alice"]);                           // --> 4
    let (name, score) = ("alice", 3);
    println!("{name} {score}");                           // --> alice 3
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A `Vec` holds one type only. `m["k"]` panics on a missing key, so use `m.get("k")`.
- An index must be `usize`: `xs[i]` with an `i32` does not compile. Use `xs[i as usize]`.
- Use a tuple where you would return a small array.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- `xs[10]` panics instead of raising `IndexError`. Negative indexes do not exist: `xs[-1]` does not compile, so use `xs.last()`.
- `m.get("k")` gives an `Option`, not `None`. Use `.unwrap_or(0)` for a default.
<!-- /from -->

<!-- from:default -->
## Key points
- `Vec` for lists, `HashMap` for maps, tuples for small fixed groups. A collection holds one element type.
- `get` returns an `Option`, so a missing item is handled in your code. Direct indexing panics.
- `HashMap` has no stable order. Sort the keys if you need one.
<!-- /from -->

## Common mistake
Calling `push` or `insert` on a collection declared without `mut`. The compiler reports E0596, and the fix is `let mut`.
