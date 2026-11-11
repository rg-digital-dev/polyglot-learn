# A typed store: generics that follow the argument

A store holds the current state and tells listeners when it changes. The typed part is the listener: `store.on("moved", e => …)` should give `e` exactly the moved event's type, worked out from the string you pass. That takes a generic parameter constrained to the event names, plus `Extract`:

```typescript
type Msg =
  | { type: "text"; body: string }
  | { type: "image"; url: string; width: number };

type MsgOf<K extends Msg["type"]> = Extract<Msg, { type: K }>;   // MsgOf<"image"> is the image message

class Inbox {
  #handlers = new Map<string, Set<(m: Msg) => void>>();

  on<K extends Msg["type"]>(type: K, handler: (m: MsgOf<K>) => void): () => void {
    const set = this.#handlers.get(type) ?? new Set();
    this.#handlers.set(type, set);
    const h = handler as (m: Msg) => void;   // stored alongside every other kind
    set.add(h);
    return () => set.delete(h);
  }

  receive(m: Msg) {
    for (const h of this.#handlers.get(m.type) ?? []) h(m);
  }
}

const inbox = new Inbox();
const stop = inbox.on("image", (m) => console.log(m.url, m.width));   // m is the image message
inbox.receive({ type: "image", url: "cat.png", width: 640 });
stop();
inbox.receive({ type: "image", url: "dog.png", width: 320 });          // nobody listening
// inbox.on("video", () => {});                                        // error: not a message type
```

`K extends Msg["type"]` means K can only be one of the names, and TypeScript infers it from the argument: `"image"`, not `string`. `Extract<Union, Shape>` keeps the members of the union that fit `Shape`.

`#handlers` is a JavaScript private field: nothing outside the class can read it, even at run time.

<!-- from:javascript -->
## Coming from JavaScript
- This is Node's `EventEmitter`, typed: `emitter.on("data", fn)` normally gives `fn` an `any`, and here it gets the exact event.
- Returning an unsubscribe function, rather than an `off(type, fn)` method, is the pattern React's `useEffect` cleanup uses.
- `#private` fields are standard JavaScript; TypeScript's `private` keyword is only checked at compile time.
<!-- /from -->

<!-- from:default -->
## Key points
- `<K extends Union["type"]>(type: K, …)` infers the exact name from the argument.
- `Extract<Union, { type: K }>` gives that name's member.
- Expose state read-only (`ReadonlyMap`, `readonly T[]`) and keep the real fields private.
<!-- /from -->

## Common mistake
Typing the handler as `(e: TaskEvent) => void`. It compiles, but every handler then has to check `e.type` again before it can use `e.to`, the check the store already made.

## In this stage
In `store.ts`, define `EventOf` and implement `TaskStore`: tighten the getters' types and `on`'s signature, as the TODOs say. The first case is the type check on `store.types.ts`.
