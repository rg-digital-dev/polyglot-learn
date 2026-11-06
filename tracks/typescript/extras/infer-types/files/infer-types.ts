// TODO: each type below is a placeholder. infer-types.types.ts says what each
// should produce; rewrite them as conditional or template literal types.

/** The element type of an array or tuple: `ElementOf<string[]>` is `string`. Anything else: `never`. */
export type ElementOf<T> = unknown;

/** The value inside any number of nested Promises: `Unwrap<Promise<Promise<number>>>` is `number`. Not a Promise: `T` itself. */
export type Unwrap<T> = T;

/** A function type's return type. Not a function: `never`. */
export type ReturnOf<F> = unknown;

/** A function type's first parameter type. Not a function: `never`. */
export type FirstArg<F> = unknown;

/** The handler name for an event: `EventName<"click">` is `"onClick"`. */
export type EventName<T extends string> = string;

/** The event a handler name is for: `EventOf<"onClick">` is `"click"`. Not starting with "on": `never`. */
export type EventOf<T extends string> = string;

/** The first element, or undefined for an empty array. */
export function first<T extends readonly unknown[]>(items: T): ElementOf<T> | undefined {
  throw new Error("TODO: implement first");
}

/** "on" followed by the event with its first letter in upper case: "click" gives "onClick". */
export function eventName<T extends string>(event: T): EventName<T> {
  throw new Error("TODO: implement eventName");
}

/** The event for a handler name: "onClick" gives "click". */
export function eventOf<T extends `on${string}`>(handler: T): EventOf<T> {
  throw new Error("TODO: implement eventOf");
}

/** A version of `fn` that calls it at most once per argument and reuses the result. */
export function memoize<F extends (arg: string) => unknown>(fn: F): (arg: FirstArg<F>) => ReturnOf<F> {
  throw new Error("TODO: implement memoize");
}
