# Classes, inheritance and metamethods

The deeper-Lua stage built a class with `__index`. This stage uses the same mechanism to build a small hierarchy: Item and Npc inherit from Entity, and Container inherits from Item. It also adds two more metamethods, so the classes behave like built-in types.

`__call` runs when a table is called like a function. Put it on a class's metatable and `Item{ id = "coin", name = "coin" }` becomes a constructor call. (A call with one table literal needs no parentheses.) `__tostring` runs when `tostring` (or `print`) meets the table.

```lua
local function class(parent)
  local c = {}
  c.__index = c                       -- instances look up methods here
  c.__tostring = function(self) return "<" .. self.label .. ">" end
  return setmetatable(c, {
    __index = parent,                  -- the class looks up its parent
    __call = function(cls, ...) return cls.new(...) end,
  })
end

local Shape = class(nil)
function Shape.new(label) return setmetatable({ label = label }, Shape) end
function Shape:area() return 0 end
function Shape:describe() return tostring(self) .. " with area " .. self:area() end

local Square = class(Shape)
function Square.new(side)
  local self = Shape.new("square")
  self.side = side
  return setmetatable(self, Square)
end
function Square:area() return self.side * self.side end

local sq = Square(3)                   -- __call
print(sq:describe())                   --> <square> with area 9
print(getmetatable(sq) == Square)      --> true
```

`describe` is found on Shape through two `__index` steps, and its `self:area()` calls Square's override, because the lookup starts again at the instance.

<!-- from:typescript -->
## Coming from TypeScript
- `__tostring` is `toString()`, and `__call` lets a class be called without `new`.
- `instanceof` doesn't exist. You can write it yourself: follow `getmetatable` from the instance to its class, and from each class to its parent.
- Fields set by the constructor live on the instance. Defaults set on the class, like `Item.weight = 1`, are found through `__index` until an instance sets its own value. That's how a class field with an initializer behaves when it is never written.
<!-- /from -->

<!-- from:default -->
## Key points
- A class needs `C.__index = C`; a subclass also needs `setmetatable(C, { __index = Parent })`.
- `__call` on the class's metatable makes the class callable.
- Lua looks up metamethods directly on the metatable, never through `__index`.
- Error level 2 blames the caller: `error(msg, 2)`.
<!-- /from -->

## Common mistake
Expecting a subclass to inherit `__tostring`:

```lua
local Animal = { __tostring = function(a) return a.name end }
Animal.__index = Animal
local Dog = setmetatable({}, { __index = Animal })
Dog.__index = Dog
print(tostring(setmetatable({ name = "Rex" }, Dog)))   --> table: 0x55d0c8a2e6f0
```

Lua reads `__tostring` from Dog with `rawget`, so Animal's is never found. Every class that should print nicely needs its own `__tostring`. A shared function assigned in one helper does the job.

## In this stage
In `entities.lua`, turn Entity, Item, Container and Npc into classes. Each class needs `__index`, `__call` and `__tostring`, and each subclass needs its link to its parent. Then implement the constructors and methods, `is_a` and `list`. Items you make now replace the plain tables of stage 1: they still have an `id` and a `name`, so the world module takes them as they are.