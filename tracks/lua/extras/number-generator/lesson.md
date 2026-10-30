# Generators with coroutines

A coroutine is a function that can pause in the middle with `coroutine.yield(v)` and continue from that point later. `coroutine.create(f)` makes one (a value of type `thread`), and `coroutine.resume(co, ...)` runs it until the next yield. `resume` returns `true` plus the yielded values, or `true` plus the return values once the function ends, or `false` plus the error message if it raised. `coroutine.status(co)` says where it is: `"suspended"`, `"running"` or `"dead"`. That makes coroutines a natural way to write generators: the loop keeps its own state in local variables, and it only produces the next value when someone asks. `coroutine.wrap(f)` hides the bookkeeping. It returns a plain function that resumes the coroutine on each call and returns what it yielded, which is exactly the shape a generic `for` expects. Values can also go in: the arguments of the first `resume` become the function's parameters, and the arguments of every later `resume` become the return values of the `yield` that paused it.

```lua
-- A generator: each call returns the next word, then nil when done.
local function words(text)
  return coroutine.wrap(function()
    for w in text:gmatch("%a+") do
      coroutine.yield(w:lower())
    end
  end)
end

for w in words("Pack my box") do io.write(w, " ") end  --> pack my box
print()

-- Values also flow in: resume's arguments become yield's return values.
local numberer = coroutine.create(function(line)  -- 1st resume fills `line`
  local n = 0
  while line do
    n = n + 1
    line = coroutine.yield(n .. ": " .. line)    -- later resumes land here
  end
  return n .. " lines"
end)

print(coroutine.resume(numberer, "alpha"))   --> true   1: alpha
print(coroutine.resume(numberer, "beta"))    --> true   2: beta
print(coroutine.status(numberer))            --> suspended
print(coroutine.resume(numberer))            --> true   2 lines
print(coroutine.status(numberer))            --> dead
```

<!-- from:typescript -->
## Coming from TypeScript
- `coroutine.wrap(function() ... coroutine.yield(v) ... end)` is Lua's `function*` generator. Calling the result is like `gen.next().value`.
- `yield` can be called from any function the coroutine calls, not only from the generator body itself.
- Sending a value in works like `gen.next(value)`: `coroutine.resume(co, value)` makes the paused `yield` return `value`.
- There is no `done` flag. A wrapped generator signals the end by returning nothing, which is `nil`, and that is what stops a `for ... in` loop.
<!-- /from -->

<!-- from:default -->
## Key points
- `create` + `resume` give you `true/false` and the status; `wrap` gives a plain function that raises on errors.
- A generator built with `wrap` works directly in `for v in gen do`.
- An endless loop inside a coroutine is fine: it only runs when resumed.
- The first `resume` passes the function's arguments; later ones become `yield`'s results.
<!-- /from -->

## Common mistake
Calling a wrapped generator again after it has finished. A `for` loop stops at the first `nil`, but a hand-written loop has to stop too:

```lua
local g = coroutine.wrap(function() coroutine.yield(1) end)
print(g())   --> 1
print(g())   --> (nothing: the function ended)
print(g())   --> raises "input:4: cannot resume dead coroutine"
```

## In this exercise
Implement four functions in `number-generator/number_generator.lua`. `range(from, to, step)` and `naturals()` return generators built with `coroutine.wrap`; `range` returns `nil, "message"` for a zero step. `take(gen, n)` collects up to n values without calling the generator more often than needed. `running_total()` returns a coroutine from `coroutine.create` that adds up the numbers sent to it with `resume`.
