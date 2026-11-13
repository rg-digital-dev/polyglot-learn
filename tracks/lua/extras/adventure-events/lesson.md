# Events with closures and weak tables

A closure is a function together with the local variables it uses from where it was written. Lua keeps those variables alive for as long as the function exists. An event emitter is built almost entirely from closures. `on` returns a function that removes the listener it just added, and that function still knows which list and which entry to remove, long after `on` has returned.

```lua
local function counter()
  local n = 0                      -- an upvalue, one per call of counter()
  return function() n = n + 1; return n end, function() n = 0 end
end

local next_a, reset_a = counter()
local next_b = counter()
print(next_a(), next_a(), next_b())   --> 1   2   1
reset_a()
print(next_a())                       --> 1
```

A listener that belongs to an object brings a problem. If the emitter holds the object, the object can never be freed while the emitter lives. A *weak table* doesn't count as a reference for the garbage collector. With `__mode = "v"` its values are weak, with `"k"` its keys. Once nothing else refers to an object, the collector takes it, and it disappears from the weak table.

```lua
local cache = setmetatable({}, { __mode = "v" })
do
  local big = { name = "temporary" }
  cache.item = big
end                                   -- big goes out of scope here
collectgarbage()
print(cache.item)                     --> nil
```

<!-- from:typescript -->
## Coming from TypeScript
- Closures work like JavaScript's: a function captures the variables in scope, by reference. Each call of the outer function makes new ones.
- `__mode = "v"` is a `WeakRef` for every value in the table. `__mode = "k"` is a `WeakMap`, keyed by objects.
- `...` forwards any number of arguments, like a rest parameter spread into the next call: `fn(...)`.
<!-- /from -->

<!-- from:default -->
## Key points
- Return a closure to undo what a function did. It keeps what it needs to know as upvalues.
- Iterate over a copy of the listener list when listeners may change it.
- A weak table lets you refer to an object without keeping it alive.
<!-- /from -->

## Common mistake
Removing from a list while `ipairs` walks it:

```lua
for i, entry in ipairs(list) do
  entry.fn()          -- if this removes list[i], the next entry moves to i
end                   -- and the loop skips it
```

`table.remove` shifts everything after the removed entry down by one, so the loop jumps over its neighbour. Copy the list (`table.move(list, 1, #list, 1, {})`) and loop over the copy.

## In this stage
In `events.lua`, implement `new` and the emitter's `on`, `once`, `count`, `emit` and `watch`, then `wrap`. A vetoing listener returns `false, reason`. Returning `nil` (or nothing) is not a veto. The game in stage 8 uses "before" listeners to block moves and "after" listeners to react to them.