# Cleanup in TypeScript

The classic tool is `try { ... } finally { ... }`: the `finally` block runs when the `try` is left, by `return` or by an exception. Since Node 24, `using` does the same with less code: `using r = open(name)` calls `r[Symbol.dispose]()` at the end of the block. For a group whose size you don't know in advance, put them on a `DisposableStack`: `stack.use(value)` adds one and returns it, and `using stack = new DisposableStack()` disposes of all of them, newest first, when the block ends.

```typescript
function withLocks(names: string[], work: () => void): void {
  using stack = new DisposableStack();
  for (const name of names) {
    stack.use(lock(name)); // if lock() throws, the ones already taken are released
  }
  work(); // and here they're released whether work() returns or throws
}
```

<!-- from:javascript -->
## Coming from JavaScript
- `using`, `DisposableStack` and `Symbol.dispose` are JavaScript (Explicit Resource Management), not TypeScript-only; TypeScript adds the `Disposable` type.
- `try`/`finally` still works everywhere and is what older code uses.
- Don't both `close()` and `using` the same value: it's then closed twice.
<!-- /from -->

<!-- from:default -->
## Key points
- `using x = ...` disposes of `x` at the end of the block, on every way out.
- `DisposableStack` for a dynamic number of resources.
- `try`/`finally` when you can't use `using`.
<!-- /from -->

## Common mistake
`map` with a throwing callback and no cleanup:

```typescript
const opened = names.map(open); // if the third open throws, the first two are never closed
```

## In this exercise
`open` (from `resource.ts`) returns a resource with `read()`, `close()` and `Symbol.dispose`. `readAll` opens them all on a `DisposableStack`, then reads each; the stack closes everything however `readAll` ends. TypeScript in Practice's `disposable-resource` goes further.
