# Lists, Maps and Tuples in Lua

Lua has one collection, the table, and it plays every role. A list is a table with keys `1, 2, 3...`: `{10, 20, 30}`. A map is a table with other keys: `{alice = 3}`, or `["bob smith"] = 5` for keys that are not plain names. Lists start at **1**, so `xs[1]` is the first item and `xs[0]` is `nil`. `#xs` is the length, and `xs[#xs + 1] = v` or `table.insert(xs, v)` adds to the end. Reading a missing key gives `nil`, and setting a key to `nil` removes it. Tables are shared, not copied: `b = a` makes two names for one table. There are no tuples; use a small table (`{name = "alice", score = 3}`) or return several values.

```lua
local xs = {10, 20, 30}
print(xs[1], xs[0], xs[4], #xs)           --> 10   nil   nil   3
xs[#xs + 1] = 40
table.insert(xs, 50)
print(#xs, table.remove(xs), #xs)         --> 5   50   4
local m = {alice = 3, ["bob smith"] = 5}
m.carol = 7
print(m.alice, m["bob smith"], m.nobody)  --> 3   5   nil
m.alice = nil
print(m.alice)                            --> nil
local a = {1, 2}; local b = a; b[1] = 99
print(a[1], a == {99, 2})                 --> 99   false
```

<!-- from:python -->
## Coming from Python
- Indexes start at 1, and `xs[0]` is `nil` with no error. Negative indexes do not count from the end.
- A missing key gives `nil`, not a `KeyError`. There is no `in` test: write `m.key ~= nil`.
- `{1, 2} == {1, 2}` is `false`. Tables compare by identity, not by contents.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Arrays and objects are the same thing. `{1, 2}` and `{x = 1}` are both tables, and you can mix them.
- `xs.length` becomes `#xs`, and `xs.push(v)` becomes `table.insert(xs, v)`.
- Object keys are written with `=`, not `:`.
<!-- /from -->

<!-- from:default -->
## Key points
- A table is both list and map. Lists use keys `1..n`.
- Indexes start at 1, `#xs` is the length, and a missing key is `nil`.
- Setting a key to `nil` deletes it, and `#` is only reliable when the list has no `nil` holes.
- `b = a` shares the table; changes through either name show in both.
<!-- /from -->

## Common mistake
Putting `nil` inside a list, as in `{1, nil, 3}`. The table has a hole, and `#` is not guaranteed to count past it. Keep lists dense, or track the length yourself.
