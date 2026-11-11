# Async feeds with async generators

Events don't always come from a file you can read at once. They arrive from a socket, a log being written, or another process's output, a little at a time. An async generator (`async function*`) yields values as they become available, and `for await` consumes them, waiting between values without blocking anything else.

```typescript
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

async function* ticks(count: number): AsyncGenerator<number> {
  for (let i = 1; i <= count; i++) {
    await sleep(10);       // waiting for the next value
    yield i;
  }
}

async function* doubled(source: AsyncIterable<number>): AsyncGenerator<number> {
  for await (const n of source) yield n * 2;   // a lazy step, like a sync generator
}

const controller = new AbortController();
for await (const n of doubled(ticks(100))) {
  console.log(n);
  if (n >= 6) controller.abort();
  if (controller.signal.aborted) break;        // break also stops ticks() and doubled()
}
```

Steps chain like synchronous generators, and each one pulls the next value only when its consumer asks for one, so a fast source can't flood a slow consumer. A `break` in the `for await` loop runs each generator's `return()`, closing the whole chain, including any files or sockets those generators opened in `try`/`finally` blocks.

`AbortController` and `AbortSignal` are the standard way to cancel async work in JavaScript. `fetch`, timers and streams all accept a signal, and checking `signal.aborted` makes your own loops cancellable the same way.

<!-- from:javascript -->
## Coming from JavaScript
- This is plain modern JavaScript; TypeScript only adds `AsyncGenerator<T>` and `AsyncIterable<T>` to say what flows through.
- Node's `readline` interface and readable streams are async iterables, so `for await (const line of rl)` feeds straight into `parseEvents`.
- Pull-based async generators handle backpressure for you, which event-emitter streams (`on("data")`) make you handle yourself.
<!-- /from -->

<!-- from:default -->
## Key points
- `async function*` plus `yield` for values that arrive over time; `for await` to consume them.
- Chain small lazy steps: chunks to lines, lines to events, events to the store.
- Check `signal.aborted` in the loop and `break`; the generators upstream are closed for you.
<!-- /from -->

## Common mistake
Splitting each chunk on `"\n"` and treating every piece as a line. A chunk can end halfway through a line, so the line arrives cut in two and both halves fail to parse. Keep the unfinished end and put it in front of the next chunk.

## In this stage
In `feed.ts`, implement `linesOf`, `parseEvents` and `follow`. That's the library done: tasks, events, a store with undo, queries, saving and live updates, every part of it typed.
