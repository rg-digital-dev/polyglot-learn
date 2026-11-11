// A store that holds the board, keeps the event log and tells listeners.

import { apply, emptyBoard, type Board, type Result } from "./board.ts";
import type { EventType, TaskEvent } from "./events.ts";

// TODO: the event whose type is K: EventOf<"moved"> should be the moved event.
// (Hint: Extract.) store.types.ts checks it.
export type EventOf<K extends EventType> = TaskEvent;

export class TaskStore {
  constructor(board: Board = emptyBoard) {
    throw new Error("TODO: implement TaskStore");
  }

  // TODO: read-only from outside: store.types.ts checks that neither can be changed.
  get board(): Map<any, any> {
    throw new Error("TODO: implement board");
  }

  get log(): TaskEvent[] {
    throw new Error("TODO: implement log");
  }

  /** Apply the event; on success log it, then call its type's handlers, then onAny's. */
  dispatch(event: TaskEvent): Result<Board> {
    throw new Error("TODO: implement dispatch");
  }

  // TODO: the handler should get exactly the event of `type`, and `type` should
  // only accept real event types.
  /** Call `handler` for every event of `type`; returns a function that stops it. */
  on(type: string, handler: (event: any) => void): () => void {
    throw new Error("TODO: implement on");
  }

  /** Call `handler` for every event, with the board after it. Returns a function that stops it. */
  onAny(handler: (event: TaskEvent, board: Board) => void): () => void {
    throw new Error("TODO: implement onAny");
  }
}
