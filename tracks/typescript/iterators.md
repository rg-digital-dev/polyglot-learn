# Iterators in TypeScript

TypeScript code usually processes collections with array methods. `filter`,
`map` and `slice` return new arrays, so they chain into a pipeline; `sort`
sorts in place and returns the same array, so it chains too.
`Object.entries(obj)` and `[...map]` turn a map into an array of `[key, value]`
pairs to start the chain. Underneath sits the iteration protocol: arrays,
strings, `Map`, `Set` and generators all work with `for...of` and spread.

```typescript
const scores: Record<string, number> = { lin: 72, ada: 90, bo: 41, cy: 72 };

const passed = Object.entries(scores)
  .filter(([, score]) => score >= 50)
  .sort(([nameA, a], [nameB, b]) => b - a || nameA.localeCompare(nameB))
  .map(([name, score]) => `${name}=${score}`);

console.log(passed); // [ 'ada=90', 'cy=72', 'lin=72' ]
```

<!-- from:javascript -->
## Coming from JavaScript
- The methods are the same. TypeScript infers the pair types, so
  `Object.entries` on a `Record<string, number>` gives `[string, number][]`
  and destructured names are typed.
- `sort` mutates the array; `toSorted` (ES2023) returns a sorted copy.
- Node 22+ also has lazy iterator helpers:
  `map.keys().filter(f).take(3).toArray()`. The workspace's `tsconfig.json`
  includes their types (`ESNext.Iterator`).
<!-- /from -->

<!-- from:default -->
## Key points
- Build the pipeline step by step: pairs, then sort, then slice, then map.
- A comparator returns a negative, zero or positive number; `||` falls through
  to a second key when the first one ties.
- `slice(0, n)` is safe when `n` is larger than the array.
<!-- /from -->

## Common mistake
Subtracting strings in a comparator. `"b" - "a"` is `NaN`, which `sort` treats
like `0`, so the order is quietly left as it was. Compare strings with
`a.localeCompare(b)`, or with `a < b ? -1 : a > b ? 1 : 0`.

## In this exercise
Implement `export function topWords(text: string, n: number): string[]` in
`top-words/top-words.ts`. Count the words, then try writing the rest as one
chain.
