# Closures

A closure is a function that remembers the variables around it after the code that created it has returned. That lets a function carry private state:

```text
c = make_counter(10, 5)
c() -> 10
c() -> 15     (the counter remembered `current` between calls)
```

Each call to `make_counter` creates a **new** variable, so two counters never share state. That's what the "counters are independent" case checks.

Languages differ in **how** the variable is captured:

- **By reference, automatically:** JavaScript/QML, Lua (as "upvalues") and Go. Mutating the captured variable just works.
- **By value unless you ask:** PHP's `fn () =>` copies variables in. To mutate, write `function () use (&$current)` to capture by reference.
- **Checked by the compiler:** a Rust closure that mutates captured state is `FnMut`. Returning it needs `move`, so the closure owns the variable rather than borrowing a local that is about to disappear.

## Where closures show up

- Callbacks and event handlers (QML signal handlers, Node callbacks).
- Iterator adapters: `map(|x| x * factor)`.
- Configuration functions and memoisation.
- Hiding state without writing a class: a closure plus its captured variables is a tiny object with one method.

## Common mistake

Returning the value **after** updating it. The first call must return `start`, so save the current value, add `step`, then return the saved value.

## Go deeper

- [Closure (Wikipedia)](https://en.wikipedia.org/wiki/Closure_(computer_programming))
- `learn compare closures` shows the capture rules per track.
