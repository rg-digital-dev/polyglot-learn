# Conditionals in TypeScript

TypeScript has the same branching tools as JavaScript: `if`/`else`, the ternary
`a ? b : c`, `switch`, and the boolean operators `&&`, `||` and `!`. What it
adds is narrowing: inside a branch the compiler knows more about a value's type
because of the check you just made. A function that answers yes or no returns
`boolean`, and often the whole body is one boolean expression.

```typescript
export function isWeekend(day: number): boolean {
  return day === 0 || day === 6;
}

export function describe(value: number | string): string {
  if (typeof value === "number") {
    return value % 2 === 0 ? "even number" : "odd number"; // value is number here
  }
  return value.length === 0 ? "empty text" : "text"; // value is string here
}
```

<!-- from:javascript -->
## Coming from JavaScript
- The operators are identical, including `%`, which keeps the sign of the left
  operand (`-7 % 2` is `-1`, not `1`).
- There is no separate integer type: `number` covers both, so `year % 4` works
  on whole numbers exactly as in JavaScript.
- Narrowing is new: after `typeof x === "number"` the compiler treats `x` as a
  number inside that branch, and flags code that could never run.
- TypeScript reports comparisons between types that cannot overlap, such as
  comparing a `number` to a string literal, which JavaScript silently allows.
- Use `===`; `==` still works, but its coercion rules are the same traps as in
  JavaScript.
<!-- /from -->

<!-- from:default -->
## Key points
- `if`, `else if`, `else`, `switch` and `? :` are the branching forms.
- `&&` and `||` short-circuit: the right side runs only when it can change the
  result.
- `%` gives the remainder; `x % n === 0` means `x` is divisible by `n`.
- A boolean expression can be returned directly; no `if` is needed.
<!-- /from -->

## Common mistake
Wrapping a boolean in an `if` just to return `true` or `false`:

```typescript
if (n % 10 === 0) { return true; } else { return false; } // verbose
return n % 10 === 0;                                     // same result
```

## In this exercise
Implement `export function leapYear(year: number): boolean` in
`leap-year/leap-year.ts`. Return a real `boolean`; the tests use strict equality.
