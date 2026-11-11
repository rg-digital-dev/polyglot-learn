// Events arriving over time: lines of JSON from a stream, applied as they come.

import { err, ok, type Result } from "./board.ts";
import type { TaskEvent } from "./events.ts";
import { isTask } from "./persist.ts";
import { isStatus } from "./tasks.ts";
import type { TaskStore } from "./store.ts";

/** Whole lines from chunks of text that may split a line anywhere. A final newline adds no empty line. */
export async function* linesOf(chunks: AsyncIterable<string>): AsyncGenerator<string> {
  throw new Error("TODO: implement linesOf");
}

/**
 * Each non-blank line (counting from 1) as an event, or an error:
 * "line N: not JSON", or "line N: not a task event" for JSON that isn't one
 * of the five events with valid fields (a created event's task checked with
 * isTask). Lazy: reads a line only when asked for the next result.
 */
export async function* parseEvents(lines: AsyncIterable<string>): AsyncGenerator<Result<TaskEvent>> {
  throw new Error("TODO: implement parseEvents");
}

/**
 * Dispatch every event from `lines` to `store`, until the lines end or
 * `signal.aborted` is true (check it before each event). Returns how many
 * were applied, and every error in order: parseEvents' and dispatch's.
 * Only `aborted` is used, so any AbortSignal fits.
 */
export async function follow(store: TaskStore, lines: AsyncIterable<string>, signal?: { readonly aborted: boolean }): Promise<{ applied: number; errors: string[] }> {
  throw new Error("TODO: implement follow");
}
