/**
 * Yield numbers from `start` up to, but not including, `end`, moving by
 * `step`. A negative step counts down (stopping above `end`). Yield nothing
 * when `start` is already past `end`. Throw an Error for a step of 0. `end`
 * may be `Infinity`.
 */
export function* range(start: number, end: number, step: number = 1): Generator<number> {
  throw new Error("TODO: implement range");
}

/**
 * Yield the first `n` items of `items`, then stop. Pull no more items from
 * `items` than that: `items` may be endless.
 */
export function* take<T>(items: Iterable<T>, n: number): Generator<T> {
  throw new Error("TODO: implement take");
}

/** Yield `fn(item)` for each item, calling `fn` only when the next value is asked for. */
export function* map<T, U>(items: Iterable<T>, fn: (item: T) => U): Generator<U> {
  throw new Error("TODO: implement map");
}

/** Counts down from `from` to 1. Each `for...of` over it starts again from `from`. */
export class Countdown implements Iterable<number> {
  from: number;

  constructor(from: number) {
    this.from = from;
  }

  [Symbol.iterator](): Iterator<number> {
    throw new Error("TODO: implement Countdown[Symbol.iterator]");
  }
}
