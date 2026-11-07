# Tables are shared

Lua has one structure, the table, and tables are always handled by reference: `b = a` gives a second name for the same table, and so do arguments, return values and storing a table in another table. Numbers, strings and booleans are values. There's no built-in copy: loop over the table, or use `table.move(a, 1, #a, 1, {})` to copy a list's items into a new table. Copying a table of tables that way is shallow; copy each inner table for a deep copy.

```lua
local a = { {1, 2}, {3} }
local b = a                                 -- the same table
local c = table.move(a, 1, #a, 1, {})       -- a new outer table, the same rows
local d = {}
for i, row in ipairs(a) do
  d[i] = table.move(row, 1, #row, 1, {})    -- a deep copy, one row at a time
end

a[1][1] = 9
print(b[1][1], c[1][1], d[1][1])            -- 9  9  1
print(b == a, c == a, c[1] == a[1])         -- true  false  true
```

<!-- from:typescript -->
## Coming from TypeScript
- Tables behave like JavaScript objects and arrays: always shared.
- `==` on tables is identity, like `===`.
- There's no spread or `structuredClone`: copying is a loop (or `table.move` for lists).
<!-- /from -->

<!-- from:default -->
## Key points
- Every table is shared by `=`, by arguments and by return values.
- `table.move(t, 1, #t, 1, {})` copies a list one level deep.
- A deep copy copies each nested table.
<!-- /from -->

## Common mistake
Creating the row once, outside the loop:

```lua
local row, grid = {0, 0, 0}, {}
for i = 1, 2 do grid[i] = row end    -- two names for one row
grid[1][1] = 9                        -- grid[2][1] is 9 too

for i = 1, 2 do grid[i] = {0, 0, 0} end   -- fixed: a new table per pass
```

## In these exercises
`make_grid(rows, cols, fill)` builds a new row table per pass. `add_bonus(scores, name, points)` returns a new table whose lists are copies: storing `scores[key]` itself in the result shares it, so copy each list, then append to `name`'s copy.
