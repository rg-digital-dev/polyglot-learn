# Closures in Lua

Functions in Lua are values, and a function defined inside another one can use the outer function's local variables. Lua calls such captured variables upvalues. They are shared, not copied: the inner function reads and writes the variable itself, and the variable lives as long as some function still refers to it. Each call of the outer function creates new locals, so each returned function gets its own private state. This is how Lua code keeps state without objects, and it is the basis of the iterators in the next lesson.

```lua
local function make_tagger(prefix)
  local n = 0                            -- a fresh n for every call to make_tagger
  return function(name)
    n = n + 1                            -- n is an upvalue of the inner function
    return string.format("%s-%03d %s", prefix, n, name)
  end
end

local bug, feat = make_tagger("BUG"), make_tagger("FEAT")
print(bug("crash on save"))  --> BUG-001 crash on save
print(bug("typo"))           --> BUG-002 typo
print(feat("dark mode"))     --> FEAT-001 dark mode

-- Two closures created by one call share the same upvalue.
local function make_box(value)
  return function() return value end, function(v) value = v end
end
local get, set = make_box(1)
set(5)
print(get())                 --> 5
```

<!-- from:typescript -->
## Coming from TypeScript
- Closures behave as in JavaScript: variables are captured by reference, so later writes are visible to every closure that shares them.
- There is no arrow syntax. An anonymous function is `function(x) return x * 2 end`.
- A numeric or generic `for` gives each iteration its own loop variable, like `let` in a JS loop.
- Forgetting `local` creates a global, which every closure in the program then shares.
<!-- /from -->

<!-- from:default -->
## Key points
- An inner function can read and change the locals of the function around it.
- Each call of the outer function creates new locals, and so new state.
- Returning several functions from one call is a simple way to share private state.
- Declare state with `local`, inside the function that creates the closure.
<!-- /from -->

## Common mistake
Keeping the state outside the factory, so every function it returns shares it:

```lua
local count = 0
local function make_ticker() return function() count = count + 1; return count end end
-- fixed: create the state inside the factory
local function make_ticker() local count = 0; return function() count = count + 1; return count end end
```

## In this exercise
Implement `function M.make_counter(start, step)` in `make-counter/make_counter.lua`. Return a function; each call returns the current value, beginning with `start`, and then advances it by `step`. The tests create one or two counters and call each several times, so the counters must not share state.
