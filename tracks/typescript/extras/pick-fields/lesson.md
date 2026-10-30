# Generics with keyof

A generic function has type parameters, written in angle brackets, that the
caller fills in, usually without noticing: TypeScript infers them from the
arguments. A constraint narrows what a type parameter may be. `K extends keyof
T` says `K` must be one of `T`'s property names, so `obj[key]` is allowed and
has the type `T[K]`, the type of that property. Together they let a helper
keep the exact types of whatever it's given instead of falling back to `any`.

```typescript
function maxBy<T>(items: T[], score: (item: T) => number): T | undefined {
  let best: T | undefined;
  for (const item of items) {
    if (best === undefined || score(item) > score(best)) best = item;
  }
  return best;
}

function sortedBy<T, K extends keyof T>(items: T[], key: K): T[] {
  return [...items].sort((a, b) => (a[key] < b[key] ? -1 : a[key] > b[key] ? 1 : 0));
}

const cities = [
  { name: "Oslo", pop: 709_000 },
  { name: "Bern", pop: 134_000 },
];
console.log(maxBy(cities, (c) => c.pop)?.name); // "Oslo", and typed as string
console.log(sortedBy(cities, "name")[0].name); // "Bern"
// sortedBy(cities, "size");
// error TS2345: Argument of type '"size"' is not assignable to parameter of type '"name" | "pop"'.
```

The editor (or `tsc`) checks the types: it rejects a key the object doesn't
have and knows what each result contains. Node only strips the types, so the
tests check what the functions return.

<!-- from:javascript -->
## Coming from JavaScript
- The bodies are plain JavaScript. The generics only change what the caller
  sees: `pick(user, ["name"])` is typed as `{ name: string }`, not `object`.
- `keyof T` is the union of a type's keys, and `T[K]` looks up a property's
  type. Both exist only at compile time; at runtime use `in` and `obj[key]`.
- Building an object key by key starts from `{}`, which TypeScript can't prove
  is a `Pick<T, K>` yet. One `as` cast on the empty object is the usual way.
<!-- /from -->

<!-- from:default -->
## Key points
- Let TypeScript infer type parameters from the arguments; don't write
  `pick<Book, "title">(...)` by hand.
- Add a constraint (`extends`) when the body needs something from the type,
  such as being able to index it with a key.
- `Pick<T, K>` and `Record<K, V>` are built-in utility types; prefer them to
  writing the result type out.
<!-- /from -->

## Common mistake
Indexing with a plain `string` instead of a constrained key:

```typescript
function get<T>(obj: T, key: string) {
  return obj[key];
  // error TS7053: Element implicitly has an 'any' type because expression of
  // type 'string' can't be used to index type 'unknown'.
}
```

Make the key its own type parameter, `<T, K extends keyof T>(obj: T, key: K)`,
and the result is `T[K]`.

## In this exercise
Implement `pick`, `pluck` and `groupBy` in `pick-fields/pick-fields.ts`. The
signatures are given; read them first, they say what the functions do. `pick`
must leave out an optional field the object doesn't have, rather than set it to
`undefined`.
