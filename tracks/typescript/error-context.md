# Error context in TypeScript

An exception travels up the stack unchanged until something catches it. To add context, catch it where you know something useful (which key, which file), and throw a new `Error` whose message adds that knowledge and whose `cause` is the original: `new Error(message, { cause: e })`. Debuggers and Node print the cause chain, and code that needs the original can read `error.cause`. Because anything can be thrown in JavaScript, a caught value is `unknown`: check `e instanceof Error` before reading `e.message`.

```typescript
function readJson(path: string, text: string): unknown {
  try {
    return JSON.parse(text);
  } catch (e) {
    const reason = e instanceof Error ? e.message : String(e);
    throw new Error(`${path}: ${reason}`, { cause: e });
  }
}

try {
  readJson("settings.json", "{ oops");
} catch (e) {
  console.log((e as Error).message); // settings.json: Expected property name or '}' in JSON at position 2 ...
  console.log((e as Error).cause instanceof SyntaxError); // true
}
```

<!-- from:javascript -->
## Coming from JavaScript
- Same API as JavaScript (`cause` arrived in ES2022). TypeScript adds that `catch (e)` gives you `unknown`, so you narrow before using it.
- `cause` is typed as `unknown` too; narrow it the same way when you read it.
- Keep the `try` block small: wrap only the call whose errors you're adding context to.
<!-- /from -->

<!-- from:default -->
## Key points
- Catch, then throw `new Error(`${context}: ${reason}`, { cause: e })`.
- Narrow `unknown` with `instanceof Error` before reading `.message`.
- Put only the failing call in the `try`, so you don't wrap errors you didn't mean to.
<!-- /from -->

## Common mistake
A `try` around the whole function, with a message that hides everything:

```typescript
try {
  /* ... twenty lines ... */
} catch {
  throw new Error("could not load settings"); // which setting? why? the cause is gone
}
```

## In this exercise
`parseNumber` in `numbers.ts` throws errors that say why. `loadSetting` finds the key, calls it inside a small `try`, and rethrows with the key in front and the original as `cause`.
