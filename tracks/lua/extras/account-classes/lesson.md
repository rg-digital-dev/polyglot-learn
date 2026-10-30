# Classes and inheritance with metatables

A metatable is an ordinary table attached to another table with `setmetatable(t, mt)`. When you read a key that `t` doesn't have, Lua checks `mt.__index`: if it is a table, the lookup continues there. That one rule is the whole class system. The class is a table of methods with `Class.__index = Class`, and each instance is a small table of fields whose metatable is the class, so methods are stored once and shared. Inheritance adds one more step to the chain: give the child class a metatable whose `__index` is the parent, and a lookup goes instance, child, parent. To override a method, define it on the child. To call the parent's version from the override, name it and pass `self` yourself: `Parent.method(self, ...)`.

```lua
local Counter = {}
Counter.__index = Counter

function Counter.new(start)
  return setmetatable({ value = start or 0 }, Counter)
end
function Counter:step()
  self.value = self.value + 1
  return self.value
end
function Counter:label()
  return "count " .. self.value
end

-- Capped inherits from Counter: lookups go instance -> Capped -> Counter.
local Capped = setmetatable({}, { __index = Counter })
Capped.__index = Capped

function Capped.new(start, cap)
  local self = Counter.new(start)       -- reuse the parent constructor
  self.cap = cap
  return setmetatable(self, Capped)     -- then re-label it as a Capped
end
function Capped:step()                  -- override...
  if self.value >= self.cap then return nil, "at the cap" end
  return Counter.step(self)             -- ...and call the parent's version
end

local c = Capped.new(8, 9)
print(c:step())        --> 9
print(c:step())        --> nil   at the cap
print(c:label())       --> count 9        (found on Counter)
print(rawget(c, "label"), getmetatable(c) == Capped)  --> nil   true
```

<!-- from:typescript -->
## Coming from TypeScript
- `class Capped extends Counter` becomes `setmetatable(Capped, { __index = Counter })`. It's the same idea as JavaScript's prototype chain, built by hand.
- There is no `super`. `super.step()` is written `Counter.step(self)`; `Counter:step()` would pass the class itself as `self`.
- There is no `constructor` either. `new` is a plain function, and a child's `new` usually calls the parent's `new` and then swaps the metatable.
- Fields live on the instance, methods on the class. `rawget(obj, key)` reads only the instance and skips `__index`, like `hasOwnProperty`.
<!-- /from -->

<!-- from:default -->
## Key points
- `__index` set to a table makes missing keys fall through to that table.
- Class: `C.__index = C`; instance: `setmetatable(fields, C)`.
- Subclass: `setmetatable(Child, { __index = Parent })` plus `Child.__index = Child`.
- Call an overridden parent method as `Parent.method(self, ...)`.
<!-- /from -->

## Common mistake
Forgetting to link the child class to the parent. The child has its own `__index`, so its instances find the child's methods, but nothing they inherit:

```lua
local Capped = {}
Capped.__index = Capped
-- c:label()  --> attempt to call a nil value (method 'label')

local Capped = setmetatable({}, { __index = Counter })   -- fixed
Capped.__index = Capped
```

## In this exercise
Complete `M.Account` in `account-classes/account_classes.lua` (`new`, `:deposit`, `:withdraw`, `:describe`), then make `M.SavingsAccount` inherit from it. The savings class adds `:add_interest()` and overrides `:withdraw` and `:describe`, and both overrides reuse the Account versions. Invalid deposits and withdrawals return `nil, "message"` instead of raising. The tests check that methods are found through the metatables, not copied onto instances or onto the child class.
