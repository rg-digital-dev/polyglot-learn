# Reading files in parallel with scoped threads

Reading ten files one after another wastes time waiting on the disk. Threads let several reads run at once. `thread::scope` makes that simple, because the threads inside it may borrow local variables: the scope doesn't return until every thread has finished.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

fn main() {
    let words = vec!["alpha", "beta", "gamma", "delta", "epsilon"];
    let next = AtomicUsize::new(0);
    // One slot per input, so results stay in input order.
    let slots: Vec<Mutex<Option<usize>>> = words.iter().map(|_| Mutex::new(None)).collect();

    thread::scope(|s| {
        for _ in 0..3 {
            s.spawn(|| loop {
                // Each thread takes the next index until none are left.
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(word) = words.get(i) else { break };
                *slots[i].lock().unwrap() = Some(word.len());
            });
        }
    }); // every thread has finished here

    let lengths: Vec<usize> = slots.into_iter().map(|m| m.into_inner().unwrap().unwrap()).collect();
    println!("{lengths:?}"); // [5, 4, 5, 5, 7]
}
```

The threads borrow `words`, `next` and `slots` without `Arc` or `move`. A thread spawned with `thread::spawn` would need `'static` data, because it might outlive the function.

An `AtomicUsize` is a counter that threads can share safely without a lock. It hands out work, so a fast thread just takes more files. `Mutex<Option<T>>` slots give each result a place of its own, so the output order doesn't depend on which thread finished first.

<!-- from:typescript -->
## Coming from TypeScript
- `thread::scope` is closest to `await Promise.all(files.map(read))`, but these are real threads running at once on several cores, not callbacks taking turns on one.
- There's no shared-memory race to worry about by convention: the compiler refuses to let two threads change the same data unless it's behind a `Mutex` or an atomic.
- Results still come back in input order, as with `Promise.all`, because each one has its own slot.
<!-- /from -->

<!-- from:default -->
## Key points
- `thread::scope` lets threads borrow from the function; it waits for them all.
- Share work with an atomic counter; store results by index to keep their order.
- Limit the threads to `jobs` (and to the number of files): more threads than work only costs.
<!-- /from -->

## Common mistake
Pushing results into one shared `Vec` as threads finish. The order then changes from run to run, and so does logq's output. Give each input its own slot, or sort the results afterwards.

## In this stage
In `src/parallel.rs`, implement `FileResult::read` and `process_files`. Then change `run` to send the files between `-`s through `process_files` with `args.jobs`, so `--jobs 4` reads four at a time.
