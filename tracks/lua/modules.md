# Modules in Lua

A Lua module is a file that returns a value, almost always a table of functions. There is no `export` keyword. What you put in the returned table is public, and a `local` function stays private to the file. `require("name")` loads a module: it searches the templates in `package.path` (a `;`-separated list such as `./?.lua`, where `?` is replaced by the name and dots in the name become directory separators), runs the file once, and caches the result in `package.loaded`. Requiring the same name again returns the same table without running the file again. Because the search uses `package.path` and not the requiring file's location, `./?.lua` means "relative to the current working directory".

```lua
-- geometry.lua
local M = {}

local function square(x) return x * x end    -- local: private to this file

function M.distance(x1, y1, x2, y2)
  return math.sqrt(square(x2 - x1) + square(y2 - y1))
end

return M                                      -- this table is what require returns
```

```lua
-- main.lua, run as `lua main.lua` from the same folder
local geometry = require("geometry")          -- found through "./?.lua" in package.path
print(geometry.distance(0, 0, 3, 4))          --> 5.0
print(geometry.square)                        --> nil
print(require("geometry") == geometry)        --> true: cached in package.loaded
```

<!-- from:typescript -->
## Coming from TypeScript
- `require` is an ordinary function call at run time, not a static `import`. Its argument is a module name, not a path: write `require("words")`, not `require("./words.lua")`.
- There are no named or default exports. `local split = require("words").split` is the closest thing to `import { split }`.
- Resolution depends on `package.path` and the current directory, not on where the requiring file lives.
- Each module is loaded once per program, like an ES module.
<!-- /from -->

<!-- from:default -->
## Key points
- A module file builds a table, fills it with functions and ends with `return M`.
- Keep helpers `local` unless other files need them.
- `require` finds files through the `?` templates in `package.path`.
- A second `require` of the same name returns the cached table.
<!-- /from -->

## Common mistake
Forgetting the `return` at the end of the module file. `require` then returns `true`:

```lua
local words = require("words")
words.split("Hi")    -- attempt to index a boolean value (local 'words')
-- fixed: end words.lua with `return M` (or return a table such as { split = split })
```

## In this exercise
This exercise has two files. Implement `split(text)` in `slugify/words.lua`, which returns `{ split = ... }`, and then `function M.slugify(title)` in `slugify/slugify.lua`, which already loads it with `local words = require("words")`. The test adds its own folder to `package.path`, so the `require` works wherever you start `lua`. The `split` tests run first.
