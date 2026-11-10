# Variables and Constants in TypeScript

TypeScript has `let` for a name you will reassign and `const` for one you will not. Prefer `const` and reach for `let` only when the value changes. Both are scoped to the nearest `{ … }` block. `var` is the old form and is scoped to the whole function, so avoid it. `const` fixes the name, not the value it points to: you can still change the contents of a `const` array or object.

```typescript
let count = 1;
count = 2;

const names = ["Ada"];
names.push("Bob"); // fine: same array, one more item
console.log(count, names); // --> 2 [ 'Ada', 'Bob' ]

if (true) {
  var leaky = "visible";
  let tidy = "hidden";
}
console.log(leaky); // --> visible
// console.log(tidy); // tsc: error TS2304 Cannot find name 'tidy'
```

<!-- from:python -->
## Coming from Python
- Declare a name with `let` or `const` the first time. A bare `x = 1` is an error in tsc.
- `if` and `for` bodies are blocks with their own scope for `let` and `const`. A Python variable made inside an `if` is still there afterwards; a TypeScript one is not.
- Python has no real constants. `const` does stop reassignment.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- The defaults are reversed: `let` is the changeable one, and there is no `mut`.
- `const` blocks reassignment only. `const xs = [1]; xs.push(2)` is allowed, where Rust would refuse.
- You can't redeclare a name in the same scope, so `let x = x * 2` (shadowing) is an error. A new block can reuse a name.
<!-- /from -->

<!-- from:php -->
## Coming from PHP
- No `$`. Declare a name once with `let` or `const`, then use it as it is.
- Block scope exists: a `let` inside an `if` is gone after it, unlike a PHP variable.
<!-- /from -->

<!-- from:default -->
## Key points
- `const` by default, `let` when the value changes, avoid `var`.
- Reassigning a `const` is error TS2588 in tsc and a `TypeError` when Node runs the file.
- `const` does not freeze arrays or objects.
- Types are usually inferred from the first value: `let count = 1` is a `number`.
<!-- /from -->

## Common mistake
Writing `let` for everything. tsc can't then tell you when you reassign a name by accident. Start with `const` and change to `let` only where the code needs it.
