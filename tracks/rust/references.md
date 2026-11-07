# Ownership instead of sharing

Rust has no hidden sharing. `let b = a;` *moves* a `Vec`: `b` owns it now, and using `a` afterwards is a compile error. To keep both, `let b = a.clone();` makes a full, independent copy (a `Vec<Vec<i64>>` clones every inner `Vec`). To let a function look at a value without taking it, pass a reference: `&a` to read, `&mut a` to change. The borrow checker allows one `&mut` *or* any number of `&` at a time, so two parts of the program can't change one `Vec` behind each other's backs.

```rust
fn main() {
    let a = vec![vec![1, 2], vec![3]];
    let mut b = a.clone();       // a deep copy
    b[0].push(9);
    println!("{a:?} {b:?}");     // [[1, 2], [3]] [[1, 2, 9], [3]]

    let c = a;                   // moved: `a` can't be used any more
    // println!("{a:?}");        // error[E0382]: borrow of moved value: `a`
    println!("{}", total(&c));   // borrowed: `c` is still ours
}

fn total(v: &[Vec<i64>]) -> i64 {
    v.iter().flatten().sum()
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `let b = a` doesn't give you a second name for one array: it *moves* the array, and `a` is gone.
- `.clone()` is `structuredClone`: it always copies all the way down.
- `&v` is how you pass without giving away; `&mut v` is the only way a function can change your `Vec`.
- Sharing with mutation is opt-in and visible in the type: `Rc<RefCell<T>>` or `Arc<Mutex<T>>`.
<!-- /from -->

<!-- from:default -->
## Key points
- Assignment and passing by value move; `.clone()` copies.
- `&T` borrows to read, `&mut T` to change; never both at once.
- Shared, changeable data needs `Rc<RefCell<T>>` (or `Arc<Mutex<T>>` across threads).
<!-- /from -->

## Common mistake
Cloning to silence the borrow checker when a borrow would do:

```rust
fn total(v: Vec<Vec<i64>>) -> i64 { v.iter().flatten().sum() }
let t = total(grid.clone());   // copies everything just to read it

fn total(v: &[Vec<i64>]) -> i64 { v.iter().flatten().sum() }
let t = total(&grid);          // fixed: borrow it
```

## In these exercises
Both pass as soon as they're correct: the rows from `vec![vec![fill; cols]; rows]` are clones, and `add_bonus` borrows `scores` (`&HashMap`) but must return an owned map, so it has to `clone()`. Notice what the other tracks need to get right that the compiler does for you here.
