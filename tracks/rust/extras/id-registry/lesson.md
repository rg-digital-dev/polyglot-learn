# Statics: global state without unsafe

A `static` is one value for the whole program, and any thread can reach it, so Rust only lets you share it if that's safe: a `static` must be `Sync`, and `static mut` needs `unsafe` for every access. The safe ways to change a global are types that handle concurrent access themselves. **Atomics** (`AtomicU64`, `AtomicBool`, …) change a number in one indivisible step: `fetch_add` reads and adds at once, so two threads can never get the same result. **`OnceLock<T>`** starts empty and can be set exactly once; every later `set` fails, and `get` hands out a `&'static T`. **`LazyLock<T>`** wraps a closure that runs on first use, however many threads arrive at once, which makes it the tool for a table or a compiled regex you only want to build once. A `static` needs a value the compiler can build without running code, which is why these types have `const fn new`.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, OnceLock};

static REQUESTS: AtomicUsize = AtomicUsize::new(0);
static START_DIR: OnceLock<String> = OnceLock::new();
static PRIMES: LazyLock<Vec<u32>> = LazyLock::new(|| {
    (2..50).filter(|n| (2..*n).all(|d| n % d != 0)).collect()
});

fn handle() -> usize {
    REQUESTS.fetch_add(1, Ordering::Relaxed) + 1
}

fn main() {
    assert_eq!(handle(), 1);
    assert_eq!(handle(), 2);

    assert!(START_DIR.set("/srv".to_string()).is_ok());
    assert!(START_DIR.set("/tmp".to_string()).is_err()); // already set
    assert_eq!(START_DIR.get().map(String::as_str), Some("/srv"));

    assert_eq!(PRIMES.len(), 15); // built here, on first use
    assert_eq!(PRIMES[0], 2);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A module-level `let counter = 0` is fine in JavaScript, which runs your code on one thread. Rust has real threads, so a plain global number would be a data race; `AtomicUsize` is the safe version.
- `LazyLock` is a lazily initialised module constant: `let cache; function getCache() { return cache ??= build(); }`, but safe when two threads ask at the same moment.
- `OnceLock` is a `const` you fill in at run time, once.
<!-- /from -->

<!-- from:default -->
## Key points
- Prefer atomics, `OnceLock` and `LazyLock` (or a `Mutex` inside a `LazyLock`) to `static mut`.
- `fetch_add` returns the value *before* the addition.
- `Ordering::Relaxed` is enough for a counter that nothing else depends on. When one thread's write must be visible together with other data, you need `Acquire`/`Release`; a `Mutex` handles that for you.
- `OnceLock::set` returns `Err(value)` when it was already set, handing your value back.
<!-- /from -->

## Common mistake
Reading and then writing an atomic in two steps:

```rust
let id = NEXT_ID.load(Ordering::Relaxed);
NEXT_ID.store(id + 1, Ordering::Relaxed); // another thread can load the same id in between
```

Each step is atomic, but the pair isn't. Use one operation that does both: `fetch_add`, or `compare_exchange` for anything more complicated.

## In this exercise
Declare four statics in `src/lib.rs`, then implement `next_id` with an atomic, `color_code` with a `LazyLock` table (and count how often it's built), and `set_app_name` and `app_name` with a `OnceLock`. The tests call them from many threads at once.
