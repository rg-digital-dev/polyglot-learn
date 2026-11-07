# Cleanup

Files, connections and locks have to be closed again, and *every* way out of a function has to do it: the normal return, the early return, and the error. Writing `close()` at the end of the function covers only the first. Each language has a construct that ties the cleanup to the scope instead, so it runs however the scope is left:

| Track | Construct | When it runs |
|---|---|---|
| Rust | `Drop` (automatic) | when the value goes out of scope; a `Vec` drops all its elements |
| Go | `defer r.Close()` | when the *function* returns, last deferred first |
| TypeScript | `using r = ...`, `DisposableStack` | at the end of the block, in reverse order |
| Python | `with r:`, `contextlib.ExitStack` | at the end of the block, last entered first |
| PHP | `try { ... } finally { ... }` | when the `try` block is left, by return or exception |
| Lua | `local r <close> = ...` | at the end of the block, for one variable |

The hard case is a *group* of resources: open three, and the third fails. The two that did open must be closed before the error goes up. Rust's `Vec`, Go's deferred calls, TypeScript's `DisposableStack` and Python's `ExitStack` handle any number; with PHP's `finally` you loop over what you opened, and in Lua you keep a table and close it on every path yourself.

## Common mistake

Closing only on the happy path:

```text
a = open("a")
b = open("b")      // fails: a is never closed
data = read(a) + read(b)
close(a); close(b)
```

## Go deeper

- `learn compare cleanup` puts every track's version side by side.
- Rust in Practice (`scoped-guard`) and TypeScript in Practice (`disposable-resource`) go further with `Drop` and `using`.
