# Discriminated unions

A union type says a value is one of several types: `A | B | C`. When every
member is an object with the same literal field, such as `kind: "circle"`, the
union is **discriminated**: checking that one field tells TypeScript which
member you have. Inside `case "circle":` of a `switch (shape.kind)`, the value
is narrowed to the circle type, and only its fields are available. In the
`default` branch nothing is left, so the value has type `never`. Passing it to
a function that only accepts `never` turns "I handled every case" into
something the compiler checks.

```typescript
type Payment =
  | { method: "card"; last4: string }
  | { method: "cash" }
  | { method: "voucher"; code: string; value: number };

function unreachable(x: never): never {
  throw new Error(`unhandled: ${JSON.stringify(x)}`);
}

function receipt(p: Payment): string {
  switch (p.method) {
    case "card":
      return `card ending ${p.last4}`; // p is the card variant here
    case "cash":
      return "cash";
    case "voucher":
      return `voucher ${p.code} (${p.value})`;
    default:
      return unreachable(p); // p: never
  }
}

console.log(receipt({ method: "voucher", code: "SPRING", value: 10 }));
```

Your editor (or `tsc`) checks the types: drop the `"cash"` case and the call to
`unreachable(p)` turns red. Node only strips the types before running, so the
tests check behaviour, including what happens when a value the types rule out
shows up anyway.

<!-- from:javascript -->
## Coming from JavaScript
- The shape is the same one you'd write in JavaScript: plain objects with a
  `type` or `kind` string. TypeScript adds that each branch knows its fields.
- The `never` check costs nothing at runtime except one throwing function, and
  it catches the classic bug of a new kind that one `switch` forgot.
- Data from outside (JSON, a cast, `any`) can still break the types. The
  `default` branch is where that ends up, so make it throw, not return
  `undefined`.
<!-- /from -->

<!-- from:default -->
## Key points
- Give every member of the union the same tag field with a different string
  literal type.
- Switch on the tag; each `case` narrows the value to one member.
- End with `default: return assertNever(x)`, so a missing case is a compile
  error and an impossible value is a runtime error.
<!-- /from -->

## Common mistake
Reading a field before narrowing:

```typescript
function size(p: Payment) {
  return p.value;
  // error TS2339: Property 'value' does not exist on type 'Payment'.
  //   Property 'value' does not exist on type '{ method: "card"; last4: string; }'.
}
```

Check `p.method === "voucher"` first, then `p.value` is allowed.

## In this exercise
In `shape-union/shape-union.ts`, implement `assertNever`, then `area`,
`describe` and `totalArea` over the `Shape` union. The last two tests cast a
`"hexagon"` into a `Shape` and expect `area` and `describe` to throw through
`assertNever`.
