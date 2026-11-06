# Cleanup with `using` and Symbol.dispose

`using` declares a variable whose value is cleaned up automatically when the block ends, however it ends: normally, through `return`, or because an exception is thrown. The value must have a `[Symbol.dispose]()` method (that's what the `Disposable` interface describes), and the runtime calls it at the end of the block. Several `using` variables in one block are disposed in reverse order, so whatever was opened last is closed first. For cleanup that has to wait (closing a connection, flushing a file), `await using` calls an `async [Symbol.asyncDispose]()` and waits for it. `DisposableStack` collects any number of disposables, for when you don't know in advance how many you'll open: `stack.use(value)` adds one, and disposing the stack disposes them all, newest first. `using` needs Node 24 or newer.

```typescript
class Timer implements Disposable {
  label: string;
  start = performance.now();

  constructor(label: string) {
    this.label = label;
  }

  [Symbol.dispose](): void {
    console.log(`${this.label}: ${Math.round(performance.now() - this.start)} ms`);
  }
}

function load(): string {
  using _timer = new Timer("load");
  if (Math.random() < 0.5) return "early"; // the timer still prints
  return "late";
}

load();
```

<!-- from:javascript -->
## Coming from JavaScript
- `using` is new JavaScript syntax (Explicit Resource Management), not just TypeScript. TypeScript adds the `Disposable` and `AsyncDisposable` interfaces that check you wrote the method.
- It replaces `try { ... } finally { thing.close() }`, and the caller can't forget it: the cleanup belongs to the value.
- Rust's `Drop` is the same idea, except that Rust drops every value at the end of its scope; here only `using` variables are disposed.
<!-- /from -->

<!-- from:default -->
## Key points
- `using x = value` calls `x[Symbol.dispose]()` when the block ends, on every way out.
- Disposal runs in reverse order of declaration.
- `await using` and `[Symbol.asyncDispose]()` for cleanup that returns a Promise; only inside `async` functions.
- `DisposableStack` holds a dynamic number of disposables; `stack.use(x)` returns `x`, so you can keep using it.
<!-- /from -->

## Common mistake
Using `const` instead of `using`, or disposing by hand and forgetting a path:

```typescript
const tx = new Transaction("save", log); // never disposed: no rollback, no commit line
if (bad) throw new Error("bad"); // with `using`, this would roll back
```

The type checker can't catch this: a `Disposable` declared with `const` is still valid code.

## In this exercise
`disposable-resource.ts` defines a `Transaction` (implement its constructor, `record`, `commit` and `Symbol.dispose`) and a `Connection` (implement `Symbol.asyncDispose`). Then use them: `transfer` with `using`, `runQuery` with `await using`, and `openAll` with a `DisposableStack`.
