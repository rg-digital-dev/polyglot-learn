# Wiring it together

The last stage adds no new mechanism. It composes the seven modules into one `Game` object. Each module stays ignorant of the others where it can: the parser knows nothing about rooms, the event emitter knows nothing about commands, and the scheduler knows nothing about games. `game.lua` is the one place that knows them all, and it is short because each piece does one thing.

Closures do most of the wiring. A verb like `quit` needs the game, but commands only receive `(state, command)`. So the game registers a handler that closes over `self`:

```lua
local Player = {}
Player.__index = Player

function Player.new(name, menu)
  local self = setmetatable({ name = name, score = 0 }, Player)
  -- The handler closes over self: menu.run doesn't need to know about players.
  menu.register("score", function()
    return self.name .. " has " .. self.score .. " points"
  end)
  return self
end
```

A world is a definition table, and the files that define worlds are Lua too: `lighthouse.lua` returns one, with functions for the keeper's conversation and for the hooks in `setup`. Loading it with `dofile` runs it with full access to everything. That's fine for a file you wrote, and it's why save files went through the sandbox instead. `main.lua` is the front end: it loads a world, prints the intro, and runs a read–eval–print loop on `io.read`.

```sh
lua main.lua                 # play the lighthouse
lua main.lua my_world.lua    # or your own world
```

<!-- from:typescript -->
## Coming from TypeScript
- `game.lua` is the composition root: the one module that imports the others and builds the object graph, the way a `main.ts` would.
- `require` caches modules in `package.loaded`, so every module that requires `world` gets the same table. Modules are singletons, like ES modules.
- `package.path` is the module resolution config: a list of `?.lua` templates that `require` tries in order. `main.lua` adds its own folder so it runs from anywhere.
<!-- /from -->

<!-- from:default -->
## Key points
- Keep modules independent and wire them together in one place.
- Use closures to give a callback what it needs, instead of widening every interface.
- Trusted configuration can be code (`dofile`); untrusted data cannot (`load` in a sandbox).
<!-- /from -->

## Common mistake
Doing the work of an earlier module again inside the game, for example matching words to items with a new loop instead of `commands.find`. The two copies drift apart, and a fix in one never reaches the other. When the game needs something, look for the module that already does it.

## In this stage
In `game.lua`, implement `new`, `on`, `spawn`, `say`, `talk`, `intro` and `input`. `item`, `has`, `save` and `load` are given. `lighthouse.lua` and `main.lua` come with the stage: once every test passes, run `lua main.lua` in this folder and play it through.