# Branded types and `satisfies`

TypeScript compares types by their shape, so two aliases of `string` are the same type: a function that wants a user ID happily accepts an order ID, or any string at all. A **brand** makes them different. Intersect the base type with a property that no real value has, `string & { readonly [brand]: "UserId" }`, and a plain string no longer fits, while a branded value still works everywhere a string does. Nothing changes at run time: the brand exists only for the type checker. The one way in is a constructor function that checks the value and then asserts the type with `as`; after that, the type itself is proof that the check happened.

`satisfies` solves a related problem. Annotating a constant (`const x: Record<string, number> = {...}`) checks it but also replaces its type, so the exact keys are forgotten. `const x = {...} satisfies Record<string, number>` checks the same thing and keeps the object's own, narrower type.

```typescript
declare const brand: unique symbol;
type Email = string & { readonly [brand]: "Email" };

function email(raw: string): Email {
  if (!raw.includes("@")) throw new Error(`not an email: ${raw}`);
  return raw as Email; // the only cast: after the check
}

function send(to: Email, text: string): string {
  return `to ${to}: ${text}`;
}

send(email("ada@example.com"), "hi");
// send("ada@example.com", "hi");  error: string is not assignable to Email

const LIMITS = { free: 5, pro: 50 } satisfies Record<string, number>;
type Plan = keyof typeof LIMITS; // "free" | "pro", not string
```

<!-- from:javascript -->
## Coming from JavaScript
- None of this exists at run time: `declare`, the brand and `satisfies` are all erased, and a `UserId` is an ordinary string in the running program.
- In JavaScript you'd validate the ID in every function that takes one, or trust callers. A brand moves the check to one place and lets the type checker enforce it everywhere else.
- `unique symbol` makes the brand's property key one that no other code can write, so an object literal can't fake it.
<!-- /from -->

<!-- from:default -->
## Key points
- `T & { readonly [brand]: "Name" }` makes a type that only branded values fit.
- One constructor per brand checks the value and casts; keep `as` out of the rest of the code.
- Arithmetic on branded numbers gives a plain `number` back, so the brand has to be put back on deliberately (`(a + b) as Cents`).
- `satisfies` checks a value against a type without widening it to that type.
<!-- /from -->

## Common mistake
Casting at the call site to make an error go away:

```typescript
loadUser(request.params.id as UserId);
```

It compiles, but it skips the check the brand exists for. Call the constructor instead (`loadUser(userId(request.params.id))`), so bad input fails where it enters.

## In this exercise
Turn the three plain aliases in `branded-ids.ts` into branded types, write the constructors that check values, then switch `PRICES` from an annotation to `satisfies`. Case 1 is the type check: learn runs `tsc` on your file and on `branded-ids.types.ts`, whose `@ts-expect-error` lines must all be type errors.
