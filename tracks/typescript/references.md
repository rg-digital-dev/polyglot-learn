# References in TypeScript

Arrays and objects are reference values: `const b = a` makes `b` point at the same array, and so does passing it to a function or storing it in another object. `===` on two arrays asks "the same array?", not "the same contents?". The spread syntax `[...a]` and `{ ...o }` copy one level (a shallow copy). `structuredClone(x)` copies everything inside (a deep copy). Numbers and strings are values: copying them is all `=` can do.

```ts
const a = [[1, 2], [3]];
const b = a;                     // the same array
const c = [...a];                // a new outer array, the same inner arrays
const d = structuredClone(a);    // new arrays all the way down

a[0]!.push(9);
console.log(b[0], c[0], d[0]);   // [ 1, 2, 9 ] [ 1, 2, 9 ] [ 1, 2 ]
console.log(b === a, c === a, c[0] === a[0]);   // true false true
```

<!-- from:javascript -->
## Coming from JavaScript
- Exactly the same rules: TypeScript's types are erased, and the copying semantics are JavaScript's.
- `readonly number[]` stops *your* code from changing an array, but it's still shared: someone with a mutable reference can change it.
<!-- /from -->

<!-- from:default -->
## Key points
- Arrays and objects are shared by `=`, by arguments and by return values.
- `[...a]`, `a.slice()` and `{ ...o }` copy one level.
- `structuredClone` copies nested arrays and objects.
<!-- /from -->

## Common mistake
`fill` puts the *same* value in every slot, and an array is a reference:

```ts
const grid = Array(2).fill(Array(3).fill(0));    // two references to one row
grid[0][0] = 9;
console.log(grid);   // [ [ 9, 0, 0 ], [ 9, 0, 0 ] ]

const fixed = Array.from({ length: 2 }, () => Array(3).fill(0));   // a new row per call
```

## In these exercises
`makeGrid(rows, cols, fill)` builds rows that don't share. `addBonus(scores, name, points)` returns a copy of `scores` with one list extended: `{ ...scores }` alone still shares the lists, so copy them too (or use `structuredClone`), and leave `scores` alone.
