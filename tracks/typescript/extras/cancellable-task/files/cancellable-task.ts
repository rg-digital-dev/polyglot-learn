/** Work that can be cancelled through the signal it's given. */
export type Task<T> = (signal: AbortSignal) => Promise<T>;

/**
 * Resolve after `ms` milliseconds. With a `signal`: reject with
 * `signal.reason` straight away if it's already aborted, and stop waiting
 * (clear the timer, reject with `signal.reason`) as soon as it's aborted.
 */
export function delay(ms: number, signal?: AbortSignal): Promise<void> {
  throw new Error("TODO: implement delay");
}

/**
 * Run `work` with a signal that aborts after `ms` milliseconds. Return its
 * result if it finishes in time. On timeout, throw
 * `Error("timed out after <ms> ms")` with the signal's reason as `cause`, even
 * if the work ignores the signal. Any other error from `work` is thrown as is.
 */
export function withTimeout<T>(work: Task<T>, ms: number): Promise<T> {
  throw new Error("TODO: implement withTimeout");
}

/**
 * Start every task with one shared signal and return the first result to
 * arrive, skipping tasks that fail. Abort the signal once it's decided, so
 * the other tasks stop. If every task fails, throw an AggregateError.
 */
export function firstOf<T>(tasks: Task<T>[]): Promise<T> {
  throw new Error("TODO: implement firstOf");
}
