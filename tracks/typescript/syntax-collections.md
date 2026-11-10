# Lists, Maps and Tuples in TypeScript

A list is an array: `number[]` (or `Array<number>`), counted from 0, with `.length`, `.push` and `.at(-1)` for the last item. A map is either a plain object (`Record<string, number>`, read with `m.alice` or `m["alice"]`) or a `Map`, which has `.get`, `.set` and `.has`. A tuple is an array typed with a fixed shape, `[string, number]`, usually unpacked by destructuring. An out-of-range index or a missing key gives `undefined`, not an error.

```typescript
const xs: number[] = [1, 2, 3];
xs.push(4);
console.log(xs.length, xs[0], xs[10], xs.at(-1)); // --> 4 1 undefined 4

const m: Record<string, number> = { alice: 3 };
console.log(m.alice, m["bob"]);                    // --> 3 undefined

const mm = new Map<string, number>([["alice", 3]]);
console.log(mm.get("alice"), mm.get("zed"), mm.has("zed")); // --> 3 undefined false

const [who, score]: [string, number] = ["alice", 3];
console.log(who, score);                           // --> alice 3

const a = [1, 2];
const b = a;        // the same array, not a copy
b.push(3);
console.log(a, [...a, 4]); // --> [ 1, 2, 3 ] [ 1, 2, 3, 4 ]
```

<!-- from:python -->
## Coming from Python
- A missing key or index gives `undefined`, never `KeyError` or `IndexError`. Negative indexes don't count from the end: use `.at(-1)`.
- An object is the dict, but its keys are strings. Use a `Map` for other key types.
- `[1] === [1]` is `false`. `b = a` shares one array, as in Python; copy with `[...a]`.
<!-- /from -->

<!-- from:go -->
## Coming from Go
- Arrays grow in place with `push`; there is no `append` to assign back.
- A missing key gives `undefined`, not a zero value. Check with `m.has(k)` on a `Map` or `k in obj`, not a `v, ok` pair.
- Assigning an array shares it, like a slice: `b = a` is not a copy.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Lists and maps are separate: a list is `[1, 2, 3]` and a map is `{ alice: 3 }`.
- Lists start at 0: `xs[0]` is the first item, and the last is `xs[xs.length - 1]` or `xs.at(-1)`.
- Length is `.length`, not `#`.
<!-- /from -->

<!-- from:default -->
## Key points
- Index from 0; `xs.length` is the count.
- `undefined` means "not there" for both a missing index and a missing key.
- A `Map` suits dynamic keys; a plain object suits fixed field names.
- Assigning an array or object shares it. Copy with `[...xs]` or `{ ...m }`.
<!-- /from -->

## Common mistake
Sorting numbers with `.sort()`. It sorts as text, so `[10, 9, 1].sort()` gives `[ 1, 10, 9 ]`. Pass a comparer: `.sort((a, b) => a - b)`.
