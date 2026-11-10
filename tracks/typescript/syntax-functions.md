# Functions in TypeScript

A function is `function name(param: Type, …): ReturnType { … }`. Write the return type: tsc then checks every path against it. Give a parameter a default with `= value`, or mark it optional with `?`. Arrow functions `(a, b) => a + b` are the short form for small functions and callbacks. A function returns one value; to return two, return an array or an object and destructure it.

```typescript
export function badge(user: string, points: number = 0): string {
  return `${user.toUpperCase()} (${points} pts)`;
}
const initials = (first: string, last: string): string => `${first[0]}${last[0]}`;

function divmod(a: number, b: number): [number, number] {
  return [Math.floor(a / b), a % b];
}
const [q, r] = divmod(7, 2);

console.log(badge("ada"), badge("ada", 3));     // --> ADA (0 pts) ADA (3 pts)
console.log(initials("Ada", "Lovelace"), q, r); // --> AL 3 1
```

<!-- from:python -->
## Coming from Python
- There is no `def` and no colon: `function` with braces. Without `return` the result is `undefined`.
- There are no keyword arguments: `badge(user, points)` is positional only. For named options, take one object parameter.
- Arrow functions replace `lambda` and can hold a full body with `{ … }`.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- The last expression is not returned. Write `return`, unless you use an arrow function with no braces.
- Return a tuple as an array typed `[number, number]`.
<!-- /from -->

<!-- from:go -->
## Coming from Go
- Types go after a colon: `a: number`, not `a int`. Each parameter needs its own type.
- One return value. For two, return an array or an object and destructure it.
- There is no capital-letter rule: visibility is `export`.
<!-- /from -->

<!-- from:default -->
## Key points
- Signature: `function name(a: A, b: B = default): Return`.
- A call with the wrong number of arguments is an error in tsc (TS2554), but not under Node.
- `return` gives the value back and the tests read it. Printing is not returning.
- Arrow form: `(a: number, b: number): number => a + b`.
<!-- /from -->

## Common mistake
Using `console.log` in place of `return`. The terminal shows the right text and the test still fails, because the function returned `undefined`.
