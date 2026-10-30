# Closures in Rust

A closure is an anonymous function, written `|args| body`, that can use variables from the scope where it was created. By default it borrows them; the `move` keyword makes it take ownership instead, which is required when the closure outlives that scope, for example when a function returns it. Every closure implements one or more of three traits: `Fn` (only reads its captures), `FnMut` (changes them) and `FnOnce` (consumes them, so it can run once). A function returns a closure as `impl Fn(...) -> T` or `impl FnMut(...) -> T`.

```rust
fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{greeting}, {name}!")
}

fn running_total() -> impl FnMut(i64) -> i64 {
    let mut total = 0;
    move |amount| {
        total += amount;
        total
    }
}

fn main() {
    let hello = make_greeter("Hello".to_string());
    assert_eq!(hello("Ada"), "Hello, Ada!");

    let mut till = running_total();
    assert_eq!(till(5), 5);
    assert_eq!(till(3), 8);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- In TS a closure always keeps captured variables alive; in Rust you decide with `move` whether it borrows or owns them.
- Each call to `running_total()` creates a new `total`, so separate closures never share state, just like separate calls to a JS factory.
- A closure that changes its state is `FnMut`, and the variable holding it must be `let mut`.
- Every closure has its own anonymous type, which is why the return type is written `impl FnMut(...)`.
<!-- /from -->

<!-- from:default -->
## Key points
- Use `move` when returning a closure; the captured values then live inside it.
- `Fn` reads, `FnMut` mutates, `FnOnce` consumes captured state.
- Call an `FnMut` through a `let mut` binding.
<!-- /from -->

## Common mistake
Returning a closure without `move`:

```rust
let mut total = 0;
|amount| { total += amount; total }
// error[E0373]: closure may outlive the current function, but it borrows `total`,
// which is owned by the current function
move |amount| { total += amount; total }   // fixed: the closure owns `total`
```

## In this exercise
Implement `pub fn make_counter(start: i64, step: i64) -> impl FnMut() -> i64`. The stub already returns a `move` closure. Each call returns the current value and then advances it by `step`, so the first call returns `start`.
