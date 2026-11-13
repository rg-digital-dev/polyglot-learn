# A dispatch table of handlers

Functions are values in Lua: you can store them in tables, pass them around and replace them. A *dispatch table* maps a name to the function that handles it. Each verb becomes a small, separate function, and nothing grows into a long `if verb == ... elseif` chain.

```lua
local handlers = {}

function handlers.add(stack, n)
  stack[#stack + 1] = n
  return "ok"
end

function handlers.pop(stack)
  if #stack == 0 then
    return nil, "the stack is empty"
  end
  return tostring(table.remove(stack))
end

local function run(stack, verb, ...)
  local handler = handlers[verb]
  if not handler then
    return nil, "unknown command: " .. verb
  end
  return handler(stack, ...)             -- passes both results on
end

local s = {}
print(run(s, "add", 4))     --> ok
print(run(s, "pop"))        --> 4
print(run(s, "pop"))        --> nil   the stack is empty
print(run(s, "dance"))      --> nil   unknown command: dance
```

The game also needs verbs that only one game has, and verbs that wrap the default behaviour. Copying the defaults into each game would work, but a metatable does it without a copy: give each game state its own `handlers` table whose `__index` is the default table. Lookups that miss fall through to the defaults, and writes only change the game's own table. The same mechanism that gives an instance its class's methods here gives one game a private layer of verbs.

<!-- from:typescript -->
## Coming from TypeScript
- `handlers.pop = function(stack) ... end` is `handlers.pop = (stack) => ...`. `function handlers.pop(stack)` is sugar for the same assignment.
- A handler returning `nil, "message"` is the Lua version of returning `{ ok: false, error }`. Multiple results need no wrapping object, and `return handler(...)` forwards them all.
- `setmetatable({}, { __index = defaults })` is `Object.create(defaults)`: a new object whose prototype is the defaults.
<!-- /from -->

<!-- from:default -->
## Key points
- Store handlers in a table keyed by name and look them up at runtime.
- A layered table (`__index` to the defaults) lets one game override verbs without touching the others.
- Keep the error messages in the handlers. The caller only shows what comes back.
<!-- /from -->

## Common mistake
Calling a handler and passing on only its first result:

```lua
local text = handler(state, command)
return text            -- the error message is lost: the player sees nothing
```

Either keep both (`local text, err = handler(...)`) or return the call directly.

## In this stage
In `commands.lua`, implement `new_state`, `register`, `find`, the seven default handlers and `run`. They use `world.lua` for rooms and exits, and the entity methods from stage 3 (`matches`, `describe`, a container's `put` and `remove`) for things.