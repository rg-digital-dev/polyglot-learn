# Errors in TypeScript

TypeScript uses JavaScript's exceptions: `throw` an `Error` (or a subclass) to
signal failure, and handle it with `try`/`catch`/`finally`. Function signatures
do not list what they can throw, so callers learn it from documentation and
names. Inside `catch`, the caught value has type `unknown` under `strict`
settings, because JavaScript lets any value be thrown.

```typescript
export function parseRatio(text: string): number {
  const [a, b] = text.split("/").map(Number);
  if (b === 0) throw new RangeError(`denominator is zero in "${text}"`);
  if (Number.isNaN(a) || Number.isNaN(b)) throw new Error(`not a ratio: "${text}"`);
  return a / b;
}

try {
  parseRatio("3/0");
} catch (e) {
  const message = e instanceof Error ? e.message : String(e);
  console.error(message);
}
```

<!-- from:javascript -->
## Coming from JavaScript
- `throw`, `try`, `catch`, `finally` and the `Error` classes are unchanged.
- The catch variable is `unknown`, not `any`, under `strict`. Narrow it with
  `e instanceof Error` before reading `e.message`.
- There are no checked exceptions: nothing in `(input: string): number` tells
  the compiler or the caller that the function may throw.
- A function that always throws can be typed as returning `never`, which helps
  narrowing in the code that calls it.
<!-- /from -->

<!-- from:default -->
## Key points
- Throw `Error` objects, not strings, so a stack trace and `message` exist.
- Give each failure its own message; "not a number" and "out of range" help
  the caller in different ways.
- Catch only where you can do something useful, and narrow the caught value.
<!-- /from -->

## Common mistake
Trusting `parseInt` or `Number` to validate input. `parseInt("80a")` is `80`,
`Number("")` is `0`, `Number("+80")` is `80` and `Number("0x50")` is `80`. Check
the characters yourself first, for example with a regular expression:

```typescript
if (!/^[0-9]+$/.test(s)) throw new Error(`not a number: "${s}"`);
```

## In this exercise
Implement `export function parsePort(input: string): number` in
`parse-port/parse-port.ts`. Return the port, or throw an `Error` for bad input.
