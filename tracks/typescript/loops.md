# Loops in TypeScript

Loops are plain JavaScript loops: the counting `for (let i = 0; ...)`, `for...of` for the values of an array or other iterable, `while`, plus `break` and `continue`. Arrays also offer callback methods such as `some`, `every`, `filter` and `reduce`. TypeScript's contribution is typing: the loop variable in `for (const x of xs)` gets the element type of `xs` automatically.

```typescript
export function countAbove(values: number[], threshold: number): number {
  let count = 0;
  for (const v of values) {
    if (v <= threshold) continue;
    count++;
  }
  return count;
}

export function firstNegativeIndex(values: number[]): number {
  for (let i = 0; i < values.length; i++) {
    if (values[i] < 0) return i;
  }
  return -1;
}
```

<!-- from:javascript -->
## Coming from JavaScript
- All loop forms behave exactly as in JavaScript; nothing new at runtime.
- `for...of` over a `number[]` gives you `number`s with no annotation needed.
- `for...in` still iterates keys as strings, even for arrays. TypeScript types the key as `string`, which is a hint you probably wanted `for...of`.
- With `noUncheckedIndexedAccess` on, `values[i]` has type `number | undefined`, because the compiler cannot prove `i` is in range. `for...of` avoids that.
- Callbacks are typed too: in `xs.some((x) => ...)` the `x` is inferred.
<!-- /from -->

<!-- from:default -->
## Key points
- Use `for...of` when you need each value, and a counting `for` when you need the index or a numeric range.
- `continue` skips to the next iteration; `break` leaves the innermost loop.
- `some` returns `true` as soon as one element passes its test.
- Declare the counter with `let` and loop values with `const`.
<!-- /from -->

## Common mistake
Using `for...in` on an array. It yields the indexes as strings, so arithmetic on them goes wrong:

```typescript
for (const i in [10, 20]) console.log(i + 1);  // "01", "11"
for (const v of [10, 20]) console.log(v + 1);  // 11, 21
```

## In this exercise
Implement `export function sumOfMultiples(factors: number[], limit: number): number` in `sum-of-multiples/sum-of-multiples.ts`. Each multiple counts once.
