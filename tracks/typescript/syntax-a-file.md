# A File at a Glance in TypeScript

An exercise is one `.ts` file. The stub is a single exported function with a doc comment above it, and its body throws `Error("TODO: implement …")` until you replace it. Comments are `// …` or `/* … */`; the stubs use the `/** … */` form to describe the function. Semicolons end statements. They are optional, but the course writes them. `export` makes the function visible to the test file, which imports it by name.

```typescript
/**
 * Return a greeting for name, trimmed of surrounding whitespace, falling back
 * to "world" when nothing is left.
 */
export function greet(name: string): string {
  return `Hello, ${name.trim() || "world"}!`;
}

console.log(greet("  Ada ")); // --> Hello, Ada!
console.log(greet("   "));    // --> Hello, world!
```

<!-- from:javascript -->
## Coming from JavaScript
- This is the syntax you know. The only new parts of the stub are the types: `name: string` and the `: string` before the `{`.
- The test file uses `import`, so your function needs `export`.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Braces `{ … }` mark the body, not indentation. Indent anyway, for readers.
- A function is only visible to other files if you write `export`; Python exposes every top-level name.
<!-- /from -->

<!-- from:default -->
## Key points
- Keep the first line of the stub (`export function greet(name: string): string`) and replace the body.
- Node 24+ runs `.ts` files by stripping the type annotations and running the rest as JavaScript.
- `console.log(x)` prints a value while you explore. The tests only read what the function returns.
- Names are camelCase: `orderTotal`, `discountPercent`.
<!-- /from -->

## Common mistake
Dropping `export`. The file still runs, but the test file cannot import the function and fails.
