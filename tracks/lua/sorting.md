# Collections and sorting in Lua

A list in Lua is a table with keys `1, 2, ..., n`, called a sequence. `#t` gives its length, `t[#t + 1] = v` appends, and `table.insert`, `table.remove`, `table.concat` and `table.move` cover the common operations. `table.sort(t, comp)` sorts in place and returns nothing. Without `comp` it uses `<`; with one, `comp(a, b)` must return `true` exactly when `a` should come before `b`. The sort is not stable. There is no slice method, so a partial copy is a loop or a `table.move` call.

```lua
local players = {
  { name = "kim", level = 7 },
  { name = "ava", level = 9 },
  { name = "lou", level = 7 },
}

-- Highest level first; equal levels by name. Copy first to keep `players` intact.
local ranked = table.move(players, 1, #players, 1, {})
table.sort(ranked, function(a, b)
  if a.level ~= b.level then return a.level > b.level end
  return a.name < b.name
end)

for i, p in ipairs(ranked) do print(i, p.name, p.level) end
--> 1  ava  9
--> 2  kim  7
--> 3  lou  7
print(players[1].name)   --> kim (unchanged)
```

<!-- from:typescript -->
## Coming from TypeScript
- `table.sort` mutates and returns nothing, like `arr.sort()` without the return value. There is no `toSorted`.
- The comparator returns a boolean ("a before b?"), not a negative/zero/positive number.
- Numbers sort numerically by default; there is no string-conversion surprise like JS `[10, 9].sort()`.
- `#t` is only reliable on sequences without holes. A list with a `nil` in the middle has an unspecified length.
- Indexes start at 1: the first `n` items are `t[1]` to `t[n]`, and `t[0]` is `nil`.
<!-- /from -->

<!-- from:default -->
## Key points
- `table.sort` sorts in place; copy the table first if the caller's data must not change.
- `table.move(src, 1, #src, 1, {})` makes a shallow copy of a sequence.
- A comparator must be a strict ordering: use `>` or `<`, never `>=` or `<=`.
- Build a result list by appending with `out[#out + 1] = v`.
<!-- /from -->

## Common mistake
A non-strict comparator. With repeated values it can raise `invalid order function for sorting` or give wrong results:

```lua
table.sort(t, function(a, b) return a >= b end)  -- wrong
table.sort(t, function(a, b) return a > b end)   -- fixed: descending
```

## In this exercise
Implement `function M.top_scores(scores, n)` in `top-scores/top_scores.lua`. Return a new sequence with at most `n` integers, highest first; for `n == 0` or no scores return `{}`. Don't sort `scores` itself.
