# A query builder typed with keyof

Builders like Prisma's and Drizzle's check field names and values against your data's type: misspell a column and it's a compile error. Underneath sit three operators. `keyof T` is the union of T's field names, `T[K]` is the type of field K, and `Pick<T, K>` is T with only the fields in K.

```typescript
interface Book {
  title: string;
  pages: number;
  read: boolean;
}

function pluck<T, K extends keyof T>(items: T[], key: K): T[K][] {
  return items.map((item) => item[key]);
}

function only<T, K extends keyof T>(item: T, ...keys: K[]): Pick<T, K> {
  const out = {} as Pick<T, K>;
  for (const k of keys) out[k] = item[k];
  return out;
}

const books: Book[] = [{ title: "Dune", pages: 412, read: true }];
const pages = pluck(books, "pages");          // number[]
const short = only(books[0], "title", "read"); // { title: string; read: boolean }
console.log(pages, short);
// pluck(books, "author");                    // error: "author" isn't a key of Book
// short.pages;                               // error: pages wasn't picked
```

`K extends keyof T` lets K be any one key (or several) of T, and TypeScript infers it from the arguments. A `where(key, test)` can then type `test` as `T[K] | ((value: T[K]) => boolean)`: a value of that field's type, or a test that receives one.

Each builder method returns a new query instead of changing `this`. A base query can then be extended in several directions without the branches affecting each other.

<!-- from:javascript -->
## Coming from JavaScript
- In plain JavaScript `where("staus", "done")` just matches nothing. Here it doesn't compile.
- `toSorted`, `toReversed` and `with` (ES2023) are the array methods that return a copy instead of changing the array, ideal for immutable steps.
- `Pick<T, K>` is a type for the `{ title, read }` object you'd get from destructuring.
<!-- /from -->

<!-- from:default -->
## Key points
- `K extends keyof T` for a field name, `T[K]` for its type, `Pick<T, K>` for a subset.
- Builder methods return a new builder, so queries can be shared and extended.
- Copy the input once and never change it; use `toSorted`, not `sort`.
<!-- /from -->

## Common mistake
`items.sort(...)` inside `sortBy`. `sort` changes the array in place, so the query's own items, and maybe the caller's array, come out reordered, and a later `.all()` on another branch of the query is sorted too.

## In this stage
In `query.ts`, type the methods with `keyof`, `T[K]` and `Pick` as the TODO says, and implement them. The first case is the type check on `query.types.ts`.
