# Operators with metamethods

Lua's operators only work on the types they were made for: `+` on numbers, `..` on strings and numbers, `<` on numbers or on two strings. A table can opt in by putting functions with special names in its metatable. When Lua meets `a + b` and one operand is a table, it looks for `__add` in the metatable of `a`, then of `b`, and calls `__add(a, b)`. The same goes for `__sub`, `__mul`, `__div`, `__unm` (unary minus), `__concat` (`..`), `__len` (`#`), `__eq` (`==` and `~=`), `__lt` (`<` and `>`) and `__le` (`<=` and `>=`). `__tostring` isn't an operator, but `tostring`, `print` and `string.format("%s")` use it. The arguments arrive in the order they were written, so in `2 * v` the number comes first.

```lua
-- Durations in whole minutes that print as "1h05".
local Dur = {}
Dur.__index = Dur

function Dur.new(minutes) return setmetatable({ min = minutes }, Dur) end

Dur.__add = function(a, b) return Dur.new(a.min + b.min) end
Dur.__eq  = function(a, b) return a.min == b.min end
Dur.__lt  = function(a, b) return a.min < b.min end
Dur.__tostring = function(d)
  return string.format("%dh%02d", d.min // 60, d.min % 60)
end
Dur.__concat = function(a, b)              -- "..": either side may be a string
  return tostring(a) .. tostring(b)
end

local talk, q_and_a = Dur.new(50), Dur.new(15)
local total = talk + q_and_a
print(total)                              --> 1h05   (print calls tostring)
print("slot: " .. total)                  --> slot: 1h05
print(total == Dur.new(65), q_and_a < talk, talk > q_and_a)  --> true  true  true
print(Dur.new(5) == 5)                    --> false  (__eq only runs for two tables)
```

<!-- from:typescript -->
## Coming from TypeScript
- TypeScript has no operator overloading; you'd write `a.add(b)`. In Lua, `a + b` on two tables calls your `__add`.
- Two different tables are never `==` by default, just like two objects in JavaScript. `__eq` changes that, but only when both operands are tables.
- `a > b` is evaluated as `b < a`, and `a >= b` as `b <= a`, so `__lt` and `__le` cover all four comparisons.
- `tostring(obj)` gives `table: 0x...` until you add `__tostring`, the counterpart of overriding `toString()`.
<!-- /from -->

<!-- from:default -->
## Key points
- Metamethods live on the metatable; with the usual class pattern that is the class table itself.
- Binary metamethods get the operands in written order, and either one may be the non-table.
- Return a new value from arithmetic metamethods instead of changing an operand.
- `table.sort` without a comparator uses `<`, so it works once `__lt` exists.
<!-- /from -->

## Common mistake
Assuming the table is always the first argument. `v * 3` calls `__mul(v, 3)`, but `3 * v` calls `__mul(3, v)`:

```lua
V.__mul = function(v, n) return V.new(v.x * n, v.y * n) end
-- 3 * v  --> attempt to index a number value (local 'v')
```

Check which argument is the number before using either one.

## In this exercise
Fill in the metamethods of `M.Vec` in `vector-ops/vector_ops.lua`: `__add`, `__sub`, `__unm`, `__mul`, `__eq`, `__lt`, `__le` and `__tostring`, plus a `:length()` method. `M.Vec.new` is provided. `*` scales a vector when one side is a number and returns the dot product when both sides are vectors. `<` and `<=` compare lengths, so `table.sort` orders vectors from shortest to longest.
