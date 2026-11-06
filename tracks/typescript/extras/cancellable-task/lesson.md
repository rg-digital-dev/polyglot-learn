# Cancelling async work with AbortSignal

A Promise can't be cancelled from outside: once work has started, only the work itself can decide to stop. `AbortController` is the standard way to ask it to. The controller owns an `AbortSignal`; you hand the signal to the work, and `controller.abort(reason)` flips `signal.aborted` to true, stores `signal.reason` and fires an `"abort"` event. Work that supports cancellation (like `fetch(url, { signal })`, or your own functions) listens for that event, stops, and rejects with the reason. `AbortSignal.timeout(ms)` gives you a signal that aborts itself with a `TimeoutError`, and `AbortSignal.any([a, b])` combines signals. When you wrap an error in your own, pass the original as `cause` (`new Error(message, { cause })`) so nothing is lost.

```typescript
function tick(times: number, signal: AbortSignal): Promise<number> {
  return new Promise((resolve, reject) => {
    let count = 0;
    const timer = setInterval(() => {
      count += 1;
      if (count === times) {
        clearInterval(timer);
        resolve(count);
      }
    }, 10);
    signal.addEventListener("abort", () => {
      clearInterval(timer);
      reject(signal.reason);
    }, { once: true });
  });
}

const controller = new AbortController();
setTimeout(() => controller.abort(new Error("user pressed stop")), 25);
try {
  await tick(100, controller.signal);
} catch (e) {
  console.log((e as Error).message); // "user pressed stop"
}
```

<!-- from:javascript -->
## Coming from JavaScript
- It's the same API as in JavaScript; TypeScript adds the types: `AbortSignal`, and `unknown` for `signal.reason`, since anything can be passed to `abort()`.
- Pass `{ once: true }` to `addEventListener` so a long-lived signal doesn't keep a listener per task.
- `Error`'s `cause` is typed as `unknown`; narrow it with `instanceof` before using it.
<!-- /from -->

<!-- from:default -->
## Key points
- `AbortController` sends the cancel request; the `AbortSignal` carries it into the work.
- Check `signal.aborted` (or call `signal.throwIfAborted()`) before starting, then listen for `"abort"`.
- Cancelling is cooperative: clear your timers and stop your loops yourself, or the work keeps running after the caller has given up.
- `AbortSignal.timeout(ms)` aborts with a `DOMException` named `"TimeoutError"`.
<!-- /from -->

## Common mistake
Racing a timeout against the work without telling the work:

```typescript
await Promise.race([work(), delay(1000).then(() => { throw new Error("timeout"); })]);
```

The caller gets its error after a second, but `work()` keeps running (and holding connections or timers) in the background. Give the work a signal and abort it.

## In this exercise
`cancellable-task.ts` asks for `delay`, which stops waiting when its signal aborts; `withTimeout`, which aborts slow work and throws an error whose `cause` is the timeout; and `firstOf`, which returns the fastest result and then cancels the rest.
