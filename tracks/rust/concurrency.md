# Concurrency in Rust

The standard library runs code in parallel with operating-system threads. `std::thread::scope` opens a scope in which `s.spawn(|| ...)` starts a thread; the scope does not return until every thread in it has finished, so those threads may borrow local data. Each `spawn` returns a handle, and `handle.join()` waits for that thread and gives back its result as a `Result` (an `Err` means the thread panicked). The borrow checker refuses code that would let two threads change the same data without synchronisation. Rust also has `async`/`await`, but that needs a runtime crate; plain threads need nothing extra.

```rust
use std::thread;

fn main() {
    let text = String::from("one two three\nfour five\nsix");

    let (words, lines) = thread::scope(|s| {
        // both threads borrow `text`; the scope waits for them
        let words = s.spawn(|| text.split_whitespace().count());
        let lines = s.spawn(|| text.lines().count());
        (words.join().unwrap(), lines.join().unwrap())
    });

    assert_eq!((words, lines), (6, 3));
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `Promise.all` runs tasks concurrently on one thread; Rust threads really run in parallel on several cores.
- A blocking call such as `thread::sleep` only blocks its own thread, so no `await` is needed.
- `join()` is the closest thing to `await`: it waits for one thread and returns its value.
- Rust needs no runtime for this; `tokio` and friends are only for `async` code.
<!-- /from -->

<!-- from:default -->
## Key points
- `thread::scope` lets spawned threads borrow from the enclosing function.
- Start every thread first, then join them; joining right after spawning makes the work sequential.
- Joining the handles in the order you spawned them keeps the results in input order.
<!-- /from -->

## Common mistake
Spawning and joining in one lazy chain:

```rust
jobs.iter().map(|&j| s.spawn(move || slow(j))).map(|h| h.join().unwrap()).collect()
// iterators are lazy: each thread is joined before the next is spawned, so it runs one at a time
let handles: Vec<_> = jobs.iter().map(|&j| s.spawn(move || slow(j))).collect(); // fixed: spawn all
```

## In this exercise
Implement `pub fn run_all(delays: &[i64]) -> Vec<i64>` using the provided `task(ms)`; do not change `task`. The tests check that the call takes about as long as the longest delay, not the sum.
