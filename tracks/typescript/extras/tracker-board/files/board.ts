// The board, and how events change it, without ever changing it in place.

import { assertNever, taskIdOf, type TaskEvent } from "./events.ts";
import { checkTitle, type Task, type TaskId } from "./tasks.ts";

export type Board = ReadonlyMap<TaskId, Task>;

/** A value, or an error: failures you return instead of throw. */
export type Result<T, E = string> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly error: E };

export const ok = <T>(value: T): Result<T, never> => ({ ok: true, value });
export const err = <E>(error: E): Result<never, E> => ({ ok: false, error });

export const emptyBoard: Board = new Map();

/**
 * The board after `event`, as a new Map: `board` and its tasks never change.
 * Changed tasks are new frozen objects; untouched ones are shared. Errors:
 *   "<id> already exists"    created, but the id is taken
 *   "no task <id>"           any other event for an id that isn't there
 *   "<id> is already <to>"   moved to the status it has
 *   checkTitle's message     renamed to a bad title
 */
export function apply(board: Board, event: TaskEvent): Result<Board> {
  throw new Error("TODO: implement apply");
}

/** Apply every event in order from `start`; the first that fails stops it, with its index. */
export function replay(events: Iterable<TaskEvent>, start: Board = emptyBoard): Result<Board, { index: number; error: string }> {
  throw new Error("TODO: implement replay");
}
