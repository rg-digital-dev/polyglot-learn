# Blocks and Control Flow in TypeScript

Blocks are braces. `if (cond) { … } else if (cond) { … } else { … }` needs parentheses around the condition. Loops are `for (let i = 0; i < n; i++)`, `for (const x of list)` for values, `while` and `do … while`. `break` leaves a loop and `continue` skips to the next round. `switch` compares with `===` and falls through to the next case unless you `break` or `return`.

```typescript
function size(day: number): string {
  switch (day) {
    case 1:
      console.log("one");   // no break: falls into case 2
    case 2:
      return "two";
    default:
      return "other";
  }
}
console.log(size(1)); // --> one  then  two

const m = { a: 1, b: 2 };
for (const k in m) console.log(k);                  // --> a  b
for (const i in ["x", "y"]) console.log(typeof i);  // --> string  string
for (const x of ["x", "y"]) console.log(x);         // --> x  y
for (const [k, v] of Object.entries(m)) console.log(k, v); // --> a 1  b 2
```

<!-- from:python -->
## Coming from Python
- `for…of` is your `for x in xs`. `for…in` gives the keys (the indexes, as strings, for an array), not the values.
- Conditions need parentheses, bodies need braces, and there is no `elif`: write `else if`.
- Use `switch` where you'd use `match`, but end every case with `break` or `return`.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- `if` is a statement, not an expression. For a value, use `cond ? a : b`.
- `switch` is not exhaustive and falls through. There is no `loop`: write `while (true)`.
- `0..10` becomes `for (let i = 0; i < 10; i++)`.
<!-- /from -->

<!-- from:go -->
## Coming from Go
- `switch` falls through; Go's does not. End each case with `break` or `return`.
- There are separate loop forms: `for (;;)`, `while (…)` and `for (const x of xs)`. The `of` form gives the value directly, with no `_, x :=`.
- The condition needs parentheses.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- `continue` exists, and so does `switch` (with fallthrough).
- Lists start at 0, so loop to `i < xs.length`.
<!-- /from -->

<!-- from:default -->
## Key points
- Parentheses around the condition, braces around the body.
- `for…of` for values, `for…in` for keys.
- `switch` falls through without `break`; `default` handles the rest.
- `cond ? a : b` is the one-line choice.
<!-- /from -->

## Common mistake
Forgetting `break` in a `switch`: the matching case runs, then the next case's code runs too. End every case with `break` or `return`.
