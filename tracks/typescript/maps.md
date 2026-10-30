# Maps in TypeScript

TypeScript has two ways to map keys to values. A plain object typed as `Record<string, number>` (or `{ [key: string]: number }`) is the classic one and turns into JSON directly. A `Map<string, number>` is a real hash map with `get`, `set`, `has` and a `size`, and it accepts any key type. Both keep keys in insertion order, and you can convert with `Object.fromEntries(map)`.

```typescript
export function tally(colors: string[]): Record<string, number> {
  const seen = new Map<string, number>();
  for (const c of colors) {
    seen.set(c, (seen.get(c) ?? 0) + 1);
  }
  return Object.fromEntries(seen);
}

tally(["red", "blue", "red"]); // { red: 2, blue: 1 }
```

<!-- from:javascript -->
## Coming from JavaScript
- Objects and `Map` behave exactly as in JavaScript; TypeScript only types them.
- `Record<string, number>` says every string key maps to a number, so `counts[k]` is typed `number` even when the key is missing. Turn on `noUncheckedIndexedAccess` to get the honest `number | undefined`.
- `Map.get` is always typed `V | undefined`, which pushes you towards `?? 0` for defaults.
- `??` replaces only `null` and `undefined`; `||` would also replace a stored `0`.
<!-- /from -->

<!-- from:default -->
## Key points
- `Record<string, number>` types a plain object; `Map<string, number>` is a dedicated map class.
- Read with a default using `map.get(k) ?? 0` or `obj[k] ?? 0`.
- `Object.fromEntries` turns a `Map` (or a list of pairs) into a plain object.
- Iterate a map with `for (const [key, value] of map)`.
<!-- /from -->

## Common mistake
Counting into `{}` inherits keys from `Object.prototype`. The word `"constructor"` already "exists", so `(counts[w] ?? 0) + 1` produces a string of function source instead of `1`. Count in a `Map`, or start from `Object.create(null)`, which has no inherited keys.

```typescript
const counts: Record<string, number> = Object.create(null);
```

## In this exercise
Implement `export function wordCount(text: string): Record<string, number>` in `word-count/word-count.ts`. The tests spread your result (`{...result}`) and compare it to a plain object, so a `Map` must be converted before returning.
