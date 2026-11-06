# Drop: cleanup that always runs

When a value goes out of scope, Rust calls its `Drop` implementation, if it has one. That happens at the end of a block, on an early `return` or `?`, when you call `drop(value)` yourself, and while a panic unwinds the stack. So a type that owns something (a file, a lock, a temporary folder, an open transaction) can clean it up in `drop`, and no caller can forget to. This pattern is called RAII or a *guard*. Values in one scope are dropped in reverse order of creation, so what was set up last is undone first. A method that takes `self` by value consumes the guard: it's dropped when the method returns, and the compiler won't let anyone use it afterwards.

```rust
use std::cell::Cell;

/// Indents output while it's alive.
struct Indent<'a> {
    depth: &'a Cell<usize>,
}

impl<'a> Indent<'a> {
    fn new(depth: &'a Cell<usize>) -> Indent<'a> {
        depth.set(depth.get() + 1);
        Indent { depth }
    }
}

impl Drop for Indent<'_> {
    fn drop(&mut self) {
        self.depth.set(self.depth.get() - 1);
    }
}

fn main() {
    let depth = Cell::new(0);
    {
        let _outer = Indent::new(&depth);
        let inner = Indent::new(&depth);
        assert_eq!(depth.get(), 2);
        drop(inner); // ends early
        assert_eq!(depth.get(), 1);
    } // _outer is dropped here
    assert_eq!(depth.get(), 0);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `drop` is like `[Symbol.dispose]()` with `using`, except that it applies to every value of the type: nobody has to remember to write `using`.
- Where you'd write `try { ... } finally { cleanup() }`, Rust puts the cleanup in the type, and a `?` or a panic still runs it.
- There's no garbage collector deciding *when*: a value is dropped at a point you can see in the code, the end of its scope.
<!-- /from -->

<!-- from:default -->
## Key points
- `impl Drop for T { fn drop(&mut self) { ... } }`. You never call `drop` the method yourself; call `drop(value)` (from the prelude) to end a value early.
- Values are dropped in reverse order of creation, and a moved value is dropped by its new owner, not the old one.
- A method taking `self` consumes the value, so "commit" or "close" can't be called twice, and the value can't be used after it.
- Don't panic inside `drop`: a panic while another panic is unwinding aborts the program.
<!-- /from -->

## Common mistake
Binding a guard to `_` drops it immediately, because `_` isn't a variable:

```rust
let _ = Transaction::begin("save", &log); // begins and rolls back on this line
let _tx = Transaction::begin("save", &log); // lives to the end of the scope
```

## In this exercise
`src/lib.rs` defines a `Transaction` that writes to a shared log. Implement `begin`, `record` and `commit`, then `Drop`, which writes "commit" or "rollback". `transfer` and `batch` don't need any cleanup code of their own: the tests check that the rollback still happens on `?`, on `drop()` and during a panic.
