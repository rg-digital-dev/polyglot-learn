# Collections and sorting in TypeScript

Arrays are TypeScript's main collection. `sort` orders an array in place and
returns that same array; `toSorted` (ES2023, Node 20+) returns a sorted copy and
leaves the original alone. Both take an optional comparator `(a, b) => number`
that returns a negative number, zero or a positive number. `slice`, `map`,
`filter` and the spread syntax `[...xs]` all produce new arrays.

```typescript
interface Player { name: string; wins: number }

export function ranking(players: readonly Player[]): string[] {
  return players
    .toSorted((a, b) => b.wins - a.wins || a.name.localeCompare(b.name))
    .map((p) => p.name);
}

ranking([{ name: "Bo", wins: 2 }, { name: "Al", wins: 5 }]); // ["Al", "Bo"]
```

<!-- from:javascript -->
## Coming from JavaScript
- `sort` still compares elements as strings when you give it no comparator:
  `[10, 9, 1].sort()` is `[1, 10, 9]`. TypeScript does not warn about this.
- `readonly number[]` (or `ReadonlyArray<number>`) on a parameter makes the
  compiler reject `sort`, `push` and friends on it, so you cannot mutate the
  caller's array by accident.
- `toSorted` needs `"lib": ["es2023"]` or newer in `tsconfig.json` for the type
  checker; Node itself already has it.
- Comparators are typed: in `(a, b) => b - a` on a `number[]`, both are numbers.
<!-- /from -->

<!-- from:default -->
## Key points
- `sort` mutates and returns the same array; `toSorted` returns a new one.
- For numbers, `(a, b) => a - b` is ascending and `(a, b) => b - a` descending.
- `slice(0, n)` takes at most `n` items and never runs past the end.
- Mark parameters `readonly` when the function must not change them.
<!-- /from -->

## Common mistake
Sorting the caller's array in place. `xs.sort(...)` reorders `xs` for everyone
holding a reference to it. Sort a copy instead:

```typescript
const sorted = [...xs].sort((a, b) => a - b); // or xs.toSorted((a, b) => a - b)
```

## In this exercise
Implement `export function topScores(scores: number[], n: number): number[]` in
`top-scores/top-scores.ts`. Return a new array and leave `scores` as it was.
