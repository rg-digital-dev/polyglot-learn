// Undo and redo for any state that changes through a reducer.

import type { Result } from "./board.ts";

/**
 * A state S changed by events E through `reduce`, with undo and redo. Keeps up
 * to `limit` steps to undo (the oldest are dropped). A successful `do` clears
 * the steps to redo; a failed one changes nothing.
 */
export class History<S, E> {
  constructor(reduce: (state: S, event: E) => Result<S>, initial: S, limit = 100) {
    throw new Error("TODO: implement History");
  }

  get state(): S {
    throw new Error("TODO: implement state");
  }

  get canUndo(): boolean {
    throw new Error("TODO: implement canUndo");
  }

  get canRedo(): boolean {
    throw new Error("TODO: implement canRedo");
  }

  do(event: E): Result<S> {
    throw new Error("TODO: implement do");
  }

  /** Go back one step; false if there's nothing to undo. */
  undo(): boolean {
    throw new Error("TODO: implement undo");
  }

  /** Go forward one undone step; false if there's nothing to redo. */
  redo(): boolean {
    throw new Error("TODO: implement redo");
  }
}
