# Undo and redo with a generic class

Because the reducer never changes a state, undo needs no clever bookkeeping: keep the old states and step back to them. Written as a generic class, the same code gives undo to the board, a text editor or a counter. Its type parameters stand for whatever state and events it's used with.

```typescript
class Stack<T> {
  #items: T[] = [];
  push(item: T): void {
    this.#items.push(item);
  }
  pop(): T | undefined {
    return this.#items.pop();
  }
  get size(): number {
    return this.#items.length;
  }
}

const words = new Stack<string>();
words.push("hello");
// words.push(42);            // error: a number isn't a string

// The type arguments are usually inferred from the constructor's arguments:
class Box<T> {
  value: T;
  constructor(value: T) {
    this.value = value;
  }
}
const n = new Box(41);          // Box<number>
console.log(n.value + 1, words.size, words.pop());
```

`History<S, E>` takes a reducer `(state: S, event: E) => Result<S>`, so `new History(apply, emptyBoard)` infers S as `Board` and E as `TaskEvent`, and `history.do(…)` accepts only task events. Nothing in the class knows about tasks.

Two stacks make undo and redo: the states behind you and the states you undid. A new action makes the undone future impossible, so it clears the redo stack, just like a text editor does.

<!-- from:javascript -->
## Coming from JavaScript
- A generic class is a JavaScript class plus type parameters; the `<S, E>` disappears when the code runs.
- `new History(apply, emptyBoard)` needs no `<Board, TaskEvent>`: TypeScript infers them from the arguments, as it does for `new Map([["a", 1]])`.
- With immutable states, "save a snapshot" is just keeping a reference: no deep copy, no `structuredClone`.
<!-- /from -->

<!-- from:default -->
## Key points
- Type parameters name the types a class works with; callers fill them in, usually by inference.
- Keep the class free of the concrete types: it should work with a counter as well as a board.
- Two stacks (past and future) are all undo and redo need when states are immutable.
<!-- /from -->

## Common mistake
Checking `if (this.#past.pop() === undefined) return false`. It pops before it checks, and it breaks when a state is legitimately `undefined`. Check `this.#past.length` first, then pop.

## In this stage
Implement `History` in `history.ts`.
