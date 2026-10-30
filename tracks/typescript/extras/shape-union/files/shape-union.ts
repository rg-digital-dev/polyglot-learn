/** A shape is exactly one of these three. The `kind` field tells them apart. */
export type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "rect"; width: number; height: number }
  | { kind: "triangle"; base: number; height: number };

/**
 * Call this in the `default` branch of a switch over `shape.kind`. If every
 * kind is handled, `value` has type `never` there and the call compiles; add
 * a kind to `Shape` and forget a case, and the editor flags the call. At
 * runtime it throws an Error whose message includes `JSON.stringify(value)`.
 */
export function assertNever(value: never): never {
  throw new Error("TODO: implement assertNever");
}

/** The area of a shape. A triangle's area is half its base times its height. */
export function area(shape: Shape): number {
  throw new Error("TODO: implement area");
}

/** A short label: "circle r=2", "rect 2x3" or "triangle b=4 h=3". */
export function describe(shape: Shape): string {
  throw new Error("TODO: implement describe");
}

/** The sum of the areas of all shapes. */
export function totalArea(shapes: Shape[]): number {
  throw new Error("TODO: implement totalArea");
}
