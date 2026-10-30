# Ownership and borrowing

Every value in Rust has exactly one owner, and the value is dropped (its memory freed) when the owner goes out of scope. Passing a value to a function either **moves** it, so the function becomes the owner and the caller can't use it any more, or **borrows** it. A shared borrow, `&T`, lets any number of readers look at the value at the same time. A mutable borrow, `&mut T`, gives exactly one writer exclusive access. The compiler's borrow checker enforces these rules, so there is no garbage collector and no data race.

```rust
fn count(items: &[String]) -> usize {   // shared borrow: read only
    items.len()
}

fn tag(items: &mut Vec<String>) {       // mutable borrow: change in place
    for item in items.iter_mut() {
        item.push_str(" [seen]");
    }
}

fn consume(items: Vec<String>) -> String { // move: the function owns it now
    items.concat()
}

fn main() {
    let mut list = vec!["a".to_string(), "b".to_string()];
    assert_eq!(count(&list), 2);   // borrowed, still ours
    tag(&mut list);                // borrowed mutably, still ours
    let all = consume(list);       // moved
    assert_eq!(all, "a [seen]b [seen]");
    // println!("{list:?}");       // error[E0382]: borrow of moved value: `list`
}
```

<!-- from:typescript -->
## Coming from TypeScript
- In TypeScript every object is shared by reference and any holder can change it. In Rust the type says what a function may do: `&T` reads, `&mut T` changes, `T` takes over.
- `readonly string[]` is the closest thing to `&[String]`, but it's only a compile-time hint. A Rust `&` borrow is a guarantee: nothing else can change the value while you hold it.
- There's no way to keep using a value after you've moved it. If you need both, pass a borrow, or `.clone()` it on purpose.
<!-- /from -->

<!-- from:default -->
## Key points
- One owner at a time; the value is dropped when its owner goes out of scope.
- Any number of `&T` borrows, or exactly one `&mut T`, never both at once.
- Accept `&[T]` rather than `&Vec<T>` and `&str` rather than `&String`: the slice forms accept more callers.
<!-- /from -->

## Common mistake
Changing a vector while iterating over it:

```rust
for word in words.iter() {
    words.push(word.to_uppercase());
    // error[E0502]: cannot borrow `*words` as mutable because it is also borrowed as immutable
}
for word in words.iter_mut() {  // fixed: change each element in place
    *word = word.to_uppercase();
}
```

## In this exercise
Implement three functions in `src/lib.rs` that take the same list three ways: `total_len(&[String])` reads it, `shout(&mut Vec<String>)` changes it in place, and `into_sentence(Vec<String>)` consumes it. The tests check that a borrowed list is still usable afterwards.
