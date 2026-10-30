# Concurrency in TypeScript

JavaScript runs your code on a single thread driven by an event loop. A slow operation such as a timer, a file read or a network request does not block that thread: it returns a `Promise` at once, and the loop runs the code waiting on it when the result arrives. Many operations can be in flight together. That is concurrency (the waiting overlaps), not parallelism (two pieces of code running at the same instant). An `async` function returns a promise, and `await` pauses only that function.

```typescript
function fetchUser(id: number): Promise<string> {
  return new Promise((resolve) => setTimeout(() => resolve(`user${id}`), 100));
}

async function main(): Promise<void> {
  const t0 = Date.now();
  const oneByOne = [await fetchUser(1), await fetchUser(2)];
  const t1 = Date.now();
  const together = await Promise.all([fetchUser(3), fetchUser(4)]);
  console.log(oneByOne, t1 - t0); // [ 'user1', 'user2' ] about 200
  console.log(together, Date.now() - t1); // [ 'user3', 'user4' ] about 100
}

main();
```

<!-- from:javascript -->
## Coming from JavaScript
- Same event loop, same promises. TypeScript adds `Promise<T>`: awaiting a `Promise<number>` gives a `number`.
- An `async` function's return type is always written `Promise<T>`.
- `Promise.all` rejects as soon as one input rejects; `Promise.allSettled` waits for every input.
- CPU-heavy code still blocks the loop; real parallelism needs `worker_threads`.
<!-- /from -->

<!-- from:default -->
## Key points
- Start every operation first, then wait for all of them together.
- `await` inside a `for` loop runs the operations one after another.
- The results come back in input order, not in the order the operations finish.
<!-- /from -->

## Common mistake
`delays.forEach(async (d) => results.push(await task(d)))`. `forEach` ignores the promises its callback returns, so the outer function returns before any task has finished, and `push` would order the results by finish time anyway.

## In this exercise
`run-all/run-all.ts` provides `export function task(ms: number): Promise<number>`; leave it unchanged. Implement `export async function runAll(delays: number[]): Promise<number[]>`. The tests check the results and that the call takes about as long as the longest delay.
