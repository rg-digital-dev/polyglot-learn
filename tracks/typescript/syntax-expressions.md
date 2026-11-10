# Operators and Strings in TypeScript

TypeScript's operators are JavaScript's. Compare with `===` and `!==`, which never convert types. The looser `==` converts first, so `1 == "1"` is `true`; avoid it (tsc often rejects it too). Strings are joined with `+` or built with template literals in backticks, where `${…}` takes any expression. `&&` and `||` return one of their operands, so `name || "world"` is a default. `??` is stricter: it only falls back on `null` or `undefined`.

```typescript
console.log(1 === 1, 1 === ("1" as any)); // --> true false
console.log("Score: " + 5);               // --> Score: 5
const who = "";
console.log(`Hi, ${who || "world"}!`);    // --> Hi, world!
const count: number = 0;
console.log(count || "d", count ?? "d");  // --> d 0
console.log(7 / 2, 7 % 3, -7 % 2);        // --> 3.5 1 -1
console.log(0.1 + 0.2);                   // --> 0.30000000000000004
console.log("  ada ".trim().toUpperCase()); // --> ADA
```

<!-- from:python -->
## Coming from Python
- `"Score: " + 5` works: the number becomes text. Python raises a `TypeError`.
- Use `&&`, `||` and `!` where Python has `and`, `or` and `not`. Comparisons don't chain: `3 > 2 > 1` is `false` at run time (`true > 1`), and tsc rejects it.
- `/` is always a decimal division and there is no `//`; use `Math.floor(a / b)`. `%` keeps the sign of the left side: `-7 % 2` is `-1`.
- `[1] === [1]` is `false`: arrays and objects compare by reference, not contents.
<!-- /from -->

<!-- from:php -->
## Coming from PHP
- Join with `+` or a template, not `.`.
- `==` also converts, but by different rules. Use `===` as you would in PHP.
- `"Score: " + 5` gives `"Score: 5"`, not a `TypeError`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Not equal is `!==`, not `~=`. Joining text uses `+` or a template, not `..`.
- `0` and `""` are falsy here, unlike in Lua, so `x || "d"` replaces them. Use `??` to keep them.
<!-- /from -->

<!-- from:default -->
## Key points
- Use `===` and `!==`.
- Template literals interpolate: `` `Hi, ${name}!` ``.
- `||` and `&&` short-circuit and return an operand; `??` only skips `null` and `undefined`.
- Falsy values: `false`, `0`, `""`, `null`, `undefined`, `NaN`. Empty arrays and objects are truthy.
<!-- /from -->

## Common mistake
Using `||` for a default when `0` or `""` is a valid value. `count || 10` turns a count of `0` into `10`. Write `count ?? 10`.
