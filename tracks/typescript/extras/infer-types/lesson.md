# Conditional types and `infer`

Types can compute other types. A **conditional type**, `T extends Pattern ? Yes : No`, picks one of two types depending on whether `T` fits the pattern. Inside the pattern, `infer X` declares a type variable that TypeScript fills in from the match, so you can take a type apart: the element of an array, the value inside a Promise, a function's parameters or return type. Given a union, a conditional type is applied to each member separately (it *distributes*), and a type may refer to itself to unwrap layers. **Template literal types** do the same for strings: `` `on${Capitalize<T>}` `` builds a string type, and `` T extends `on${infer E}` `` takes one apart. The built-in helpers `ReturnType`, `Parameters` and `Awaited` are written exactly like this. All of it happens in the type checker: none of it exists when the code runs.

```typescript
type KeysOf<T> = T extends Map<infer K, unknown> ? K : never;
type Setter<T extends string> = `set${Capitalize<T>}`;
type Field<T extends string> = T extends `set${infer F}` ? Uncapitalize<F> : never;

type A = KeysOf<Map<"id" | "name", number>>; // "id" | "name"
type B = Setter<"name" | "age">; // "setName" | "setAge"
type C = Field<"setName">; // "name"

function setter<T extends string>(field: T): Setter<T> {
  return ("set" + field.slice(0, 1).toUpperCase() + field.slice(1)) as Setter<T>;
}

const s: "setName" = setter("name");
```

<!-- from:javascript -->
## Coming from JavaScript
- This is code that runs in the type checker, not in Node: a conditional type is an `if` over types, and `infer` is like destructuring a type.
- At run time, `setter("name")` is ordinary string concatenation. TypeScript can't follow `toUpperCase()` at the type level, so the function ends with one cast that tells it what the result is.
- Where JavaScript code would check `typeof`, a conditional type decides on the static type, so it's resolved before the program ever runs.
<!-- /from -->

<!-- from:default -->
## Key points
- `T extends P ? X : Y` chooses a type; `infer V` captures a part of `P` to use in `X`.
- Unions distribute: `ElementOf<A[] | B[]>` is `A | B`.
- Return `never` for "doesn't apply": it disappears from unions.
- Test types with `Expect<Equal<A, B>>`; assignability alone hides mistakes like `any`.
<!-- /from -->

## Common mistake
Forgetting that an unconstrained check accepts more than you meant:

```typescript
type ElementOf<T> = T extends (infer E)[] ? E : never;
type X = ElementOf<readonly string[]>; // never: a readonly array isn't a mutable array
```

Match `readonly (infer E)[]` instead: it covers both mutable and readonly arrays and tuples.

## In this exercise
Replace the placeholder types at the top of `infer-types.ts` with conditional and template literal types, then implement the four functions so their return types follow their arguments. Case 1 is the type check: learn runs `tsc` on your file and on `infer-types.types.ts`.
