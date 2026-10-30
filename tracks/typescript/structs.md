# Structs in TypeScript

TypeScript has no `struct` keyword. A record is a plain object, and an `interface` (or a `type` alias) describes its shape: the field names and their types. The interface exists only for the type checker. At runtime there is just the object literal, so it costs nothing to create and serialises to JSON as is. When the data also needs behaviour, a constructor or `instanceof` checks, use a `class` instead.

```typescript
interface Point {
  x: number;
  y: number;
}

interface Box {
  topLeft: Point;
  width: number;
  height: number;
}

function grow(box: Box, by: number): Box {
  return { ...box, width: box.width + by, height: box.height + by };
}

const b: Box = { topLeft: { x: 0, y: 0 }, width: 2, height: 3 };
console.log(grow(b, 1)); // { topLeft: { x: 0, y: 0 }, width: 3, height: 4 }
```

<!-- from:javascript -->
## Coming from JavaScript
- Object literals are unchanged. The interface adds a compile-time check that every field is present with the right type, and it flags misspelt extra properties in a literal.
- Typing is structural: any object with the right fields is a `Point`, however it was created.
- Interface or class? An interface is erased and has no constructor. A class makes real objects with a prototype, so `instanceof` works. Under Node's type stripping, declare class fields and assign them in the constructor; parameter properties (`constructor(public x: number)`) are rejected.
- `readonly` on a field is a compile-time rule only; `Object.freeze` is the runtime version.
<!-- /from -->

<!-- from:default -->
## Key points
- Describe a record with `interface`; build values as object literals.
- `{ ...box, width: 3 }` copies a record with some fields changed.
- Reach for a class only when you need methods, validation or `instanceof`.
<!-- /from -->

## Common mistake
Forgetting that spread copies are shallow. `{ ...box }` is a new object, but its `topLeft` is the same `Point` as the original's, so changing `copy.topLeft.x` changes both. Copy the nested record too, or use `structuredClone(box)`.

## In this exercise
Implement `export function orderTotal(items: LineItem[], discountPercent: number): OrderSummary` in `order-total/order-total.ts`; both interfaces are already in the stub. Return a plain object literal: the tests use `assert.deepStrictEqual`, which also compares prototypes, so a class instance would not match. Round the discount down with `Math.floor`.
