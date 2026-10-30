# Functions and strings in TypeScript

A TypeScript function is a JavaScript function with type annotations on its
parameters and, optionally, its return value. The compiler (or your editor)
checks every call against those annotations; at runtime the types are gone and
the code behaves exactly like plain JavaScript. Strings are immutable, and
template literals (backticks with `${...}`) are the usual way to build them.

```typescript
export function badge(user: string, points: number = 0): string {
  const label = user.toUpperCase();
  return `${label} (${points} pts)`;
}

export const initials = (first: string, last: string): string =>
  `${first[0] ?? ""}${last[0] ?? ""}`;

badge("ada");          // "ADA (0 pts)"
initials("Ada", "Lovelace"); // "AL"
```

<!-- from:javascript -->
## Coming from JavaScript
- Everything you know about functions, closures, arrow functions and string
  methods still applies; TypeScript only adds annotations on top.
- `name: string` is a compile-time promise, not a runtime check. Node strips the
  annotation and runs the code without looking at it; run `tsc --noEmit` or
  watch your editor's squiggles to actually get the check.
- Calling a function with too many or too few arguments is an error in
  TypeScript, unlike JavaScript. Mark optional parameters with `?` or give them
  a default value.
- Returning a different type on one branch is caught too, but only if you
  write the return type (`): string`) or read the inferred one in your editor.
<!-- /from -->

<!-- from:default -->
## Key points
- Annotate parameters as `name: Type` and the return as `): Type` after the
  parameter list.
- Template literals interpolate any expression: `` `Hi ${name}` ``.
- String methods like `trim`, `toUpperCase` and `slice` return new strings;
  the original is never changed.
- Types are checked by the compiler and editor, not while the program runs.
<!-- /from -->

## Common mistake
Leaving the return type off and forgetting a branch. TypeScript then infers
`string | undefined` and the bug surfaces far away. Write the return type so the
compiler points at the function itself:

```typescript
function label(n: number): string {
  if (n > 0) return `+${n}`;
  // error: Function lacks ending return statement
}
```

## In this exercise
Implement `export function greet(name: string): string` in `hello/hello.ts`.
The tests call it with plain strings and compare the result with `===`.
