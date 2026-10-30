# Shared ownership: Rc, RefCell, Arc and Mutex

Ownership normally means one owner. When a value really needs several owners, wrap it in a reference-counted pointer: `Rc<T>` on one thread, `Arc<T>` (atomic) across threads. `Rc::clone(&x)` doesn't copy the value; it adds an owner, and the value is dropped when the last owner goes. Shared values are read-only by default, so changing one needs **interior mutability**: `RefCell<T>` checks the borrow rules at run time on one thread (`borrow()`, `borrow_mut()`), and `Mutex<T>` lets one thread at a time `lock()` it. The usual pairs are `Rc<RefCell<T>>` and `Arc<Mutex<T>>`.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // One thread: two names for one mutable score list.
    let scores = Rc::new(RefCell::new(vec![10]));
    let alias = Rc::clone(&scores);
    alias.borrow_mut().push(20);
    assert_eq!(*scores.borrow(), vec![10, 20]);
    assert_eq!(Rc::strong_count(&scores), 2);

    // Several threads: each gets its own Arc to one Mutex-protected total.
    let total = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (1..=3)
        .map(|n| {
            let total = Arc::clone(&total);
            thread::spawn(move || *total.lock().unwrap() += n)
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(*total.lock().unwrap(), 6);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- In TypeScript every object is already shared, mutable and kept alive by the GC. Rust makes each of those choices visible: `Rc` for sharing, `RefCell` for mutating through a shared handle.
- JavaScript runs your code on one thread (workers share nothing by default). Rust threads really share memory, so the compiler insists on `Arc` and `Mutex` (it rejects `Rc` across threads).
- `lock().unwrap()` returns a guard; the lock is released when the guard goes out of scope, not with an explicit `unlock()`.
<!-- /from -->

<!-- from:default -->
## Key points
- `Rc::clone` and `Arc::clone` are cheap: they bump a counter, they don't copy the data.
- `RefCell` panics if you call `borrow_mut()` while another borrow is still alive; keep borrows short.
- Join every thread before reading the final result, or you may read it half-done.
<!-- /from -->

## Common mistake
Moving an `Arc` into the first thread and then trying to use it again:

```rust
for _ in 0..4 {
    thread::spawn(move || *counter.lock().unwrap() += 1);
    // error[E0382]: the type `Arc` does not implement `Copy`
    //   value moved into closure here, in previous iteration of loop
}
for _ in 0..4 {
    let counter = Arc::clone(&counter);   // fixed: each thread gets its own Arc
    thread::spawn(move || *counter.lock().unwrap() += 1);
}
```

## In this exercise
In `src/lib.rs`, build the `Rc<RefCell<..>>` shopping list and the `Shopper` methods (`add` takes `&self`), then `parallel_increments` and `parallel_word_count` with `Arc<Mutex<..>>` and one thread per unit of work.
