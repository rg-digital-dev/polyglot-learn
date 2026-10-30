# Classes and interfaces in Lua

Lua has neither classes nor interfaces, but metatables give you both patterns in a few lines. A "class" is a table that holds the methods. Setting `Class.__index = Class` and giving each instance that table as its metatable (`setmetatable(obj, Class)`) makes every lookup of a missing key on the instance fall through to the class. The colon is sugar for passing the object along: `function Class:send(msg)` defines a method with a hidden first parameter `self`, and `obj:send("hi")` means `obj.send(obj, "hi")`. An interface is just an agreement about method names. Any table that has them can be used in their place (duck typing), and a LuaLS `---@class` annotation can write the agreement down.

```lua
local Email = {}
Email.__index = Email                    -- instances look up missing keys in Email
function Email.new(address)
  return setmetatable({ address = address }, Email)
end
function Email:send(msg)                 -- same as Email.send(self, msg)
  return "mail to " .. self.address .. ": " .. msg
end

-- No class at all, but it has a send method, so it fits too.
local logger = { send = function(self, msg) return "log: " .. msg end }

-- Works with anything that has a send method; it never checks the type.
local function broadcast(notifiers, msg)
  local out = {}
  for i, n in ipairs(notifiers) do out[i] = n:send(msg) end
  return out
end

local r = broadcast({ Email.new("a@example.org"), logger }, "build passed")
print(r[1], r[2])  --> mail to a@example.org: build passed   log: build passed
```

<!-- from:typescript -->
## Coming from TypeScript
- No `class`, `new` or `implements`. `Class.new(...)` is an ordinary function that calls `setmetatable`.
- `obj:m(x)` passes `obj` as `self`; `obj.m(x)` does not. The wrong one gives a confusing "nil value" error inside the method.
- Like TypeScript's structural typing, any table with the right methods fits, but the check only happens when a method is called.
- There is no `instanceof`. `getmetatable(x) == Rect` works, but code that needs it is no longer duck-typed.
<!-- /from -->

<!-- from:default -->
## Key points
- A class is a method table with `__index` pointing to itself; `setmetatable` makes an instance.
- Define methods with `function T:name()` and call them with `obj:name()`.
- Code that takes "a shape" should call only the agreed methods.
- `table.sort` is not stable: equal elements may change order.
<!-- /from -->

## Common mistake
Expecting `table.sort` to keep equal elements in their input order. Record each element's position and use it to break ties:

```lua
table.sort(jobs, function(a, b) return a.priority > b.priority end)  -- ties may swap
```

`table.sort` isn't stable. When two items compare equal, the comparator needs something else to decide by: where each item was before sorting.

## In this exercise
Complete the `M.Rect` and `M.Square` classes in `shapes/shapes.lua` (`new`, `:area()` and `:label()`, as laid out in the stub), then implement `function M.describe_all(shapes)`. It returns a sequence of strings like `"rect 2x3: 6"`, largest area first, with equal areas in input order, and should only call `area` and `label`.
