# Records in Lua

Lua has no `struct` or `record` keyword. A record is a table with string keys, built with a table constructor such as `{ room = "attic", celsius = 31 }` and read with `r.room`. Nothing fixes its shape: you can add a field at any time, a misspelled field simply reads as `nil`, and assigning `nil` removes a field. Tools fill the gap. The lua-language-server (LuaLS) reads `---@class` and `---@field` comments and warns about unknown fields in your editor, but at run time a record is just a table. Tables are passed by reference, so a function that returns a new table is easier to reason about than one that edits its argument.

```lua
---@class Reading
---@field room string
---@field celsius integer

---@param readings Reading[]
local function summarize(readings)
  local count, total, hottest = 0, 0, nil
  for _, r in ipairs(readings) do
    count = count + 1
    total = total + r.celsius
    if hottest == nil or r.celsius > hottest.celsius then hottest = r end
  end
  return {
    count = count,
    average = count > 0 and total // count or 0,
    hottest_room = hottest and hottest.room or "",
  }
end

local s = summarize({ { room = "attic", celsius = 31 }, { room = "cellar", celsius = 12 } })
print(s.count, s.average, s.hottest_room)  --> 2   21   attic
print(7 / 2, 7 // 2, -7 // 2)              --> 3.5 3   -4
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `interface` or `type` declaration. A `---@class` comment plays that role for the editor only.
- A typo is not a compile error: `item.unitCents` on a table with `unit_cents` is `nil`, and the arithmetic that follows fails with "attempt to perform arithmetic on a nil value (field 'unitCents')".
- `/` always returns a float. `//` is floor division and keeps integers, which suits money in cents.
- `{ a = nil }` creates no key at all, unlike `{ a: undefined }`.
<!-- /from -->

<!-- from:default -->
## Key points
- Build a record with a table constructor: `{ name = value, ... }`.
- Read fields with `t.field`; a missing field is `nil`.
- `a // b` rounds down and stays an integer for integer operands.
- LuaLS annotations document a shape; nothing enforces it at run time.
<!-- /from -->

## Common mistake
Using `/` where the result must be a whole number:

```lua
local fee = amount * 3 / 100     -- 29.97 for an amount of 999
local fee = amount * 3 // 100    -- fixed: 29
```

## In this exercise
Implement `function M.order_total(items, discount_percent)` in `order-total/order_total.lua`. Each item is a table like `{ sku = "tea", qty = 2, unit_cents = 150 }`; the stub's `---@class LineItem` and `---@class OrderSummary` annotations describe both shapes. Return `{ item_count = ..., subtotal_cents = ..., discount_cents = ..., total_cents = ... }` with integer values. The tests compare tables key by key.
