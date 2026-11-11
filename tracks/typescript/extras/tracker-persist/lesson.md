# Loading untrusted data with type guards

Types are only as good as the data in them. `JSON.parse` returns whatever is in the text: a hand-edited file, an old format, or garbage. Its honest type is `unknown`, which TypeScript won't let you use until you check what it is. A type guard is a function that does the check and tells the compiler about it with `value is T`:

```typescript
interface Point {
  x: number;
  y: number;
}

const isObject = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v);

function isPoint(v: unknown): v is Point {
  return isObject(v) && typeof v.x === "number" && typeof v.y === "number";
}

function parsePoints(text: string): Point[] | string {
  let data: unknown;
  try {
    data = JSON.parse(text);
  } catch (e) {
    return `not JSON: ${(e as Error).message}`;
  }
  if (!Array.isArray(data)) return "expected an array";
  for (const [i, p] of data.entries()) {
    if (!isPoint(p)) return `[${i}]: expected a point`;
  }
  return data as Point[];
}

console.log(parsePoints('[{"x": 1, "y": 2}]'), parsePoints('[{"x": "1"}]'), parsePoints("{"));
```

After `if (isPoint(p))`, `p` is a `Point`, with no cast. A guard is a promise you make to the compiler, so it has to check everything the type says.

Errors that name the path (`tasks[2].priority`) turn "the file is broken" into a one-line fix. A version number in the format lets a later release recognise old files and convert them, instead of misreading them.

<!-- from:javascript -->
## Coming from JavaScript
- `JSON.parse(text) as Board` is the TypeScript version of trusting the file. It compiles, and the first bad field crashes somewhere far from where it was read.
- `unknown` is `any` with the safety on: you can hold it and pass it around, but you can't read fields until you've checked.
- Libraries such as Zod generate these guards from a schema; writing one by hand shows what they do.
<!-- /from -->

<!-- from:default -->
## Key points
- Data from outside is `unknown` until checked; never `as` it into shape.
- A type guard (`v is T`) checks once and narrows the type for the code after it.
- Return errors that say where the problem is, and version the format.
<!-- /from -->

## Common mistake
A guard that checks less than its type promises, such as only `typeof v.id === "string"` for a whole Task. The compiler believes the guard, so a task without a status gets through as a `Task`, and the code that relies on it breaks somewhere else entirely.

## In this stage
In `persist.ts`, implement `toJSON`, `isTask` and `parseBoard`. The docstring lists every error in the order they're checked.
