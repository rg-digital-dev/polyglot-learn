// A typed query builder: keys and values are checked against the item type.

// TODO: these signatures accept any key and any value, and select() returns
// `any`. query.types.ts checks that keys must be fields of T, that values (or
// the test function's parameter) have that field's type, and that select()
// returns objects with only the chosen fields. Use keyof, T[K] and Pick.
export class Query<T extends object> {
  /** Copies `items`, so later changes to the source don't affect the query. */
  constructor(items: Iterable<T>) {
    throw new Error("TODO: implement Query");
  }

  /** A new query keeping items whose `key` equals `test`, or passes it when it's a function. */
  where(key: string, test: unknown): Query<T> {
    throw new Error("TODO: implement where");
  }

  /** A new query sorted by `key`; null values go last either way; ties keep their order. */
  sortBy(key: string, order: "asc" | "desc" = "asc"): Query<T> {
    throw new Error("TODO: implement sortBy");
  }

  /** A new query with at most `n` items. */
  limit(n: number): Query<T> {
    throw new Error("TODO: implement limit");
  }

  /** Run the query. */
  all(): T[] {
    throw new Error("TODO: implement all");
  }

  count(): number {
    throw new Error("TODO: implement count");
  }

  /** Run the query, keeping only the given fields of each item. */
  select(...keys: string[]): any[] {
    throw new Error("TODO: implement select");
  }
}

export function query<T extends object>(items: Iterable<T>): Query<T> {
  return new Query(items);
}
