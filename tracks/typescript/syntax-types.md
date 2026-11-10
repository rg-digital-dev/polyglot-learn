# Types and Annotations in TypeScript

TypeScript adds types to JavaScript: `name: string`, `points: number`, a return type after the parameter list. The course runs the tests with Node (24+, type stripping), which deletes the annotations and does not check them. A type error does not fail a test. Only `tsc` checks types, and the course runs it for some exercises only. Treat the annotations as a contract you read, and run `tsc --noEmit --strict file.ts` yourself when you want the check.

```typescript
const n: number = JSON.parse('"hi"'); // the type system can't see into JSON
console.log(typeof n); // --> string

let s: string = 5;     // tsc: error TS2322 Type 'number' is not assignable to type 'string'
console.log(s);        // --> 5  (Node ran it anyway)

let inferred = 1;      // inferred as number
// inferred = "a";     // tsc: error TS2322

type Pair = [string, number]; // a tuple type: an array of fixed shape
const p: Pair = ["a", 1];
let u: string | undefined;    // a union: either type
console.log(u?.length, p);    // --> undefined [ 'a', 1 ]
```

<!-- from:python -->
## Coming from Python
- Node ignores annotations the way Python does. The difference is that `tsc` is a real checker and `--strict` catches a lot.
- There is one number type: `number` covers `int` and `float`. `7 / 2` is `3.5`, and there is no `//`.
- `None` becomes two values, `null` and `undefined`. A missing property or argument is `undefined`.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- Types are erased and not enforced at run time. A wrong value can still reach a typed parameter through `JSON.parse`, `any` or a cast.
- There is no `Option<T>`: write `T | undefined` (or `T | null`), and use `?.` and `??` to handle it.
- There are no integer types. Every number is a 64-bit float, so whole numbers above 2^53 lose precision.
<!-- /from -->

<!-- from:go -->
## Coming from Go
- Types are checked by `tsc`, not by the tool that runs your code. A file with type errors still runs under Node.
- Types are structural: any object with the right fields fits, with no declared "implements".
- `nil` splits into `null` and `undefined`, and there are no zero values: an unset `let x: string | undefined` is `undefined`.
<!-- /from -->

<!-- from:default -->
## Key points
- Write types as `value: Type` and the return as `): Type`. Inference fills in the rest.
- Node runs `.ts` files by stripping types. It does not check them.
- Only `tsc` reports type errors. Use `--strict` for the full check.
- Basic types: `number`, `string`, `boolean`, `null`, `undefined`, arrays `number[]`, unions `A | B`, tuples `[A, B]`.
<!-- /from -->

## Common mistake
Seeing the tests pass and assuming the types are right. `function add(a: number, b: number): number` called with `"x"` and `"y"` returns `"xy"` under Node. If the types matter, run `tsc --noEmit --strict`.
