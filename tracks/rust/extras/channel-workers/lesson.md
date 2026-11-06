# Channels, Send and Sync

"Don't communicate by sharing memory; share memory by communicating." A channel (`std::sync::mpsc`, multi-producer single-consumer) moves values from one thread to another: `send` hands the value over, and the receiving thread owns it from then on, so there's nothing to lock. A `Receiver` works as an iterator that ends once every `Sender` has been dropped, which is how a thread knows the work is done. Senders can be cloned for several producers, but there is only one receiver; several workers share it as `Arc<Mutex<Receiver<T>>>`.

The compiler checks all of this with two marker traits. A type is **`Send`** if it can be moved to another thread, and **`Sync`** if `&T` can be shared between threads. `thread::spawn` and `Sender::send` require `Send`, so values that aren't thread-safe can't leak across. `Rc` is neither (its count isn't atomic), `Arc` is both, `RefCell` is `Send` but not `Sync`, and `Mutex<T>` makes any `Send` value `Sync`. You almost never implement these traits yourself: the compiler derives them from a type's fields.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone(); // one sender per producer
        thread::spawn(move || {
            tx.send(format!("worker {id} done")).unwrap();
        });
    }
    drop(tx); // otherwise the loop below never ends

    let mut messages: Vec<String> = rx.into_iter().collect();
    messages.sort();
    assert_eq!(messages, ["worker 0 done", "worker 1 done", "worker 2 done"]);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A channel is `postMessage` between workers, with real threads and typed messages. Like `postMessage` with a transfer list, the sender gives the value up.
- JavaScript never needs `Send`/`Sync` because workers share nothing by default. Rust lets threads share memory, so the compiler checks what may cross.
- `for msg of rx` ending when the senders are gone is like an async iterator over a stream that closes.
<!-- /from -->

<!-- from:default -->
## Key points
- `send` moves the value: after it, the sending thread can't use it.
- The receiving loop ends when every `Sender` is dropped, so drop the original after cloning one into each thread.
- Workers sharing one queue: `Arc<Mutex<Receiver<T>>>`. Take the lock only for `recv()`, not while doing the work.
- An error that mentions `Rc<...>` "cannot be sent between threads safely" is `Send` doing its job: use `Arc`.
<!-- /from -->

## Common mistake
Holding the queue's lock while doing the job:

```rust
let rx = queue.lock().unwrap();      // the guard lives to the end of the loop body
while let Ok(job) = rx.recv() {
    results.send(slow_square(job)).unwrap(); // every other worker waits here
}
```

The workers take turns, and the pool is no faster than one thread. Lock, `recv`, and let the guard drop in one statement: `let job = queue.lock().unwrap().recv();`.

## In this exercise
`src/lib.rs` asks for three uses of channels: `pipeline`, two stages in their own threads; `squares`, a worker pool that shares one job queue, puts results back in order and counts finished jobs with an atomic; and `count_words`, where every thread sends back its own map instead of locking a shared one. The `squares` tests time the pool, so the workers really have to run at the same time.
