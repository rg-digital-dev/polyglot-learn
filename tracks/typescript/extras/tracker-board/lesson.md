# A reducer: new state, never changed state

A reducer takes the current state and an event and returns the next state, without changing the one it was given. It's the core of Redux and of event sourcing. It makes history cheap, because old states stay valid, and bugs rare, because nothing changes behind your back.

```typescript
type Result<T, E = string> = { ok: true; value: T } | { ok: false; error: E };

type Stock = ReadonlyMap<string, number>;
type Move = { item: string; by: number };

function apply(stock: Stock, move: Move): Result<Stock> {
  const have = stock.get(move.item) ?? 0;
  if (have + move.by < 0) return { ok: false, error: `only ${have} ${move.item} left` };
  return { ok: true, value: new Map(stock).set(move.item, have + move.by) };   // a copy
}

const before: Stock = new Map([["pens", 3]]);
const r = apply(before, { item: "pens", by: -2 });
if (r.ok) console.log(r.value.get("pens"), before.get("pens"));   // 1 3: before is unchanged
const bad = apply(before, { item: "pens", by: -9 });
if (!bad.ok) console.log(bad.error);                              // only 3 pens left
```

`ReadonlyMap` is a `Map` type without `set` or `delete`, so the compiler stops accidental changes. `new Map(old)` copies the entries, not the values in them: unchanged tasks are shared between the old and new board, which is safe because tasks are frozen.

A `Result` makes failure part of the return type. Callers must check `r.ok` before they can reach `r.value`, and that check narrows the type, just like a discriminated union. Exceptions stay for bugs; a "no such task" event is ordinary bad input.

<!-- from:javascript -->
## Coming from JavaScript
- `{ ...task, status: "done" }` is the same spread you'd use in a React state update; `readonly` makes the compiler insist on it.
- `Result` replaces `try`/`catch` around every call: the type tells callers this can fail, and they can't forget to look.
- A copy per event sounds slow, but a Map of a few thousand tasks copies in microseconds, and undo comes for free.
<!-- /from -->

<!-- from:default -->
## Key points
- Return a new board; never `set` or `delete` on the one you were given.
- Share what didn't change; replace a task with `Object.freeze({ ...task, change })`.
- Expected failures are `Result` errors, not exceptions.
<!-- /from -->

## Common mistake
`board.set(...)` on the board you were passed, with a cast to get past `ReadonlyMap`. The caller's board changes too. Undo, replay and anything else holding the old board now silently show the new state.

## In this stage
In `board.ts`, implement `apply` and `replay`. `Result`, `ok`, `err` and `emptyBoard` are given.
