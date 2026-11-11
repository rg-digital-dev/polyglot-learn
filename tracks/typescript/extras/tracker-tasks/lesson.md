# Tasks with branded IDs and literal unions

This project builds a typed task-tracker library, the kind of core a to-do app, a kanban board or a CLI would sit on. It's one folder that grows over eight stages, one module per stage. Half of the work is the types themselves: four stages come with a `*.types.ts` file that `tsc` checks, so a type that's too loose fails just like a wrong return value.

It starts with the task. Three kinds of type do most of the work:

```typescript
declare const brand: unique symbol;
type UserId = string & { readonly [brand]: "UserId" };   // a branded string

function userId(raw: string): UserId {
  if (!/^u-\d+$/.test(raw)) throw new Error(`not a user id: ${raw}`);
  return raw as UserId;                                    // the only cast
}

const SIZES = ["S", "M", "L"] as const;                    // readonly ["S", "M", "L"]
type Size = (typeof SIZES)[number];                        // "S" | "M" | "L"

interface Shirt {
  readonly owner: UserId;
  readonly size: Size;
  readonly sleeves: 0 | 1 | 2;                             // a union of literals
}

const shirt: Shirt = Object.freeze({ owner: userId("u-7"), size: "M", sleeves: 2 });
console.log(shirt, Object.isFrozen(shirt));
// shirt.size = "L";            // error: size is read-only
// const s: Size = "XL";        // error: "XL" isn't a Size
// const o: UserId = "u-7";     // error: a plain string isn't a UserId
```

A brand adds a property that no real value has, so only values that went through `userId()` fit the type. At run time it's still an ordinary string: `declare` and all the types are erased when Node runs the file.

`as const` keeps an array's exact strings instead of widening them to `string[]`. `(typeof SIZES)[number]` then turns the array into a union type, so the list of values and the type can never drift apart.

<!-- from:javascript -->
## Coming from JavaScript
- None of this exists at run time: the brand, the unions and `readonly` are all checks that `tsc` makes before the code runs.
- `readonly` is checked by the compiler; `Object.freeze` is the run-time version. Use both when the value really must not change.
- A union of string literals replaces the "magic strings" you'd otherwise check with `if (!["todo", "doing", "done"].includes(s))` everywhere.
<!-- /from -->

<!-- from:default -->
## Key points
- Brand a primitive (`string & { readonly [brand]: "TaskId" }`) when mixing up two kinds of ID would be a bug.
- `as const` plus `(typeof X)[number]` makes a union from a list of values.
- `readonly` fields, plus `Object.freeze`, for values that shouldn't change after they're made.
<!-- /from -->

## Common mistake
Writing `export type Status = string` "for now". Every `if (status === "doen")` typo then compiles, and nothing tells you until a task vanishes from a column. A literal union catches the typo at the line where you make it.

## In this stage
Tighten the types at the top of `tasks.ts`, then implement `taskId`, `isStatus`, `checkTitle`, `isDate` and `createTask`. The first case is the type check: `tasks.types.ts` lists what the types must allow and forbid.
