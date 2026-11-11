# Events as a discriminated union

An event-sourced app stores what happened, not just the current state: "t-1 was created", "t-1 moved to doing". Each kind of event carries different data. A discriminated union describes that exactly: several object types that share one literal field, the discriminant, whose value says which type you have.

```typescript
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "rect"; width: number; height: number }
  | { kind: "point" };

function assertNever(value: never): never {
  throw new Error(`unexpected: ${JSON.stringify(value)}`);
}

function area(s: Shape): number {
  switch (s.kind) {
    case "circle":
      return Math.PI * s.radius ** 2;   // here s is the circle: s.radius is allowed
    case "rect":
      return s.width * s.height;        // and here the rect
    case "point":
      return 0;
    default:
      return assertNever(s);            // every kind handled, so s is never
  }
}

console.log([{ kind: "circle", radius: 1 }, { kind: "rect", width: 2, height: 3 }].map((s) => area(s as Shape)));
```

Checking `s.kind` narrows `s` to that member, so each branch can use only the fields its kind has. The default branch is where exhaustiveness comes from. Once every kind is handled, `s` has type `never`, and passing it to `assertNever` compiles. Add a fourth shape to the union and forget its `case`, and that line becomes a type error that points at the switch to fix.

`Shape["kind"]` is the union of the discriminant's values, `"circle" | "rect" | "point"`, handy when you need the list of kinds as a type.

<!-- from:javascript -->
## Coming from JavaScript
- In JavaScript, events like these are objects with a `type` field and a comment listing the fields. Here the union is that comment, and the compiler checks it.
- `switch (event.type)` is the same code you'd write in JavaScript; the narrowing comes for free.
- `assertNever` still throws at run time, for data that bypassed the types (say, from JSON).
<!-- /from -->

<!-- from:default -->
## Key points
- One object type per event, each with a literal `type`, joined with `|`.
- Switch on the discriminant; each case narrows to one member.
- `default: return assertNever(event)` makes a missing case a compile error.
<!-- /from -->

## Common mistake
One type with every field optional (`{ type: string; id?: TaskId; to?: Status; … }`). It accepts `{ type: "moved" }` with no `to`, so every handler fills up with `!` and `?? ""`, and a renamed event with a `to` field compiles fine.

## In this stage
In `events.ts`, write `TaskEvent` as a union of the five events and `EventType` as their names, then implement `assertNever`, `taskIdOf`, `describe` and `eventsFor`. The first case is the type check on `events.types.ts`.
