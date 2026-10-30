# Narrowing unknown values

`unknown` is the type of a value you know nothing about, such as the result of `JSON.parse` or a `catch` variable. TypeScript lets you do almost nothing with it until you check what it is. Each check narrows the type inside the branch that follows: `typeof x === "string"` makes `x` a `string`, `Array.isArray(x)` makes it an array, `"id" in x` proves a property exists. A **type guard** is a function that returns `x is Something`; when it returns true, the caller's value is narrowed to `Something`. That is how untrusted data becomes typed data: check once at the edge, then the rest of the code can trust the type.

```typescript
interface Point {
  x: number;
  y: number;
}

function isPoint(v: unknown): v is Point {
  return (
    typeof v === "object" &&
    v !== null &&
    "x" in v && typeof v.x === "number" &&
    "y" in v && typeof v.y === "number"
  );
}

function readPoints(json: string): Point[] {
  const data: unknown = JSON.parse(json);
  if (!Array.isArray(data)) throw new Error("expected an array");
  return data.filter(isPoint); // Point[]: the guard narrows each element
}

console.log(readPoints('[{"x":1,"y":2},{"x":"no"},{"x":3,"y":4}]'));
// [ { x: 1, y: 2 }, { x: 3, y: 4 } ]
```

The editor (or `tsc`) checks that you narrowed before using a value. It can't check that a type guard tells the truth: `v is Point` is a promise you make. The tests check the promise by feeding in good and bad data.

<!-- from:javascript -->
## Coming from JavaScript
- The checks are the ones you already write: `typeof`, `Array.isArray`, `in`, `!== null`. TypeScript tracks their results, so after the check the editor knows the type.
- `JSON.parse` returns `any`, which turns type checking off. Assign it to an `unknown` variable straight away so you have to check it.
- `typeof null === "object"` is still true. Check `!== null` as well.
<!-- /from -->

<!-- from:default -->
## Key points
- Prefer `unknown` to `any` for data you haven't checked: `any` allows everything, `unknown` allows nothing until you narrow it.
- A guard `(v: unknown): v is T` packages a check for reuse, for example in `filter` or an `if`.
- Arrays are objects too: rule them out with `Array.isArray` when you expect a plain object.
<!-- /from -->

## Common mistake
Reading a property off `unknown` without narrowing:

```typescript
const data: unknown = JSON.parse(text);
console.log(data.name);
// error TS18046: 'data' is of type 'unknown'.
```

Check `typeof data === "object" && data !== null && "name" in data` first.

## In this exercise
In `parse-unknown/parse-unknown.ts`, implement the `isUser` type guard, then `parseUser` and `parseUsers`. Errors must say what is wrong: the field name first (`"age: expected a number"`), and for `parseUsers` the entry's index before that (`"[1] age: expected a number"`). The tests check the start of the message.
