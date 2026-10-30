/**
 * A new object with only the listed keys of `obj`. `K extends keyof T` means
 * every key must be a property of `T`, and `Pick<T, K>` is `T` cut down to
 * those keys. A key that is missing on `obj` (an optional property) is left
 * out, not set to undefined. `obj` itself is not changed.
 */
export function pick<T extends object, K extends keyof T>(obj: T, keys: K[]): Pick<T, K> {
  throw new Error("TODO: implement pick");
}

/** The value of `key` from every item, in order. `T[K]` is the type of that property. */
export function pluck<T, K extends keyof T>(items: T[], key: K): T[K][] {
  throw new Error("TODO: implement pluck");
}

/**
 * Group items under the key that `key(item)` returns, keeping their input
 * order inside each group. `PropertyKey` is `string | number | symbol`: the
 * types an object key can have.
 */
export function groupBy<T, K extends PropertyKey>(items: T[], key: (item: T) => K): Record<K, T[]> {
  throw new Error("TODO: implement groupBy");
}
