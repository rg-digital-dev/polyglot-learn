# A world made of tables

This project builds a text adventure engine in eight stages, all in one folder. Each stage adds a module, and the last one wires them into a game you can play. It starts where every Lua program starts: with tables.

Lua has one data structure. The same table is a record (`room.name`), a dictionary (`world.rooms["hall"]`) and a list (`room.items[1]`), depending on its keys. A module is a table too: a file that builds a table of functions and returns it, which `require` loads once and caches.

```lua
-- shelf.lua
local M = {}

function M.new()
  return { books = {}, by_title = {} }        -- a list and a dictionary
end

function M.add(shelf, title, author)
  if shelf.by_title[title] then
    return nil, "already on the shelf: " .. title
  end
  local book = { title = title, author = author }   -- a record
  shelf.books[#shelf.books + 1] = book               -- append
  shelf.by_title[title] = book                       -- the same table, by key
  return book
end

function M.authors(shelf)
  local seen, list = {}, {}
  for _, book in ipairs(shelf.books) do              -- in order: 1, 2, 3, ...
    if not seen[book.author] then
      seen[book.author] = true
      list[#list + 1] = book.author
    end
  end
  table.sort(list)
  return table.concat(list, ", ")
end

return M
```

```lua
local shelf = require("shelf")
local s = shelf.new()
shelf.add(s, "Dune", "Herbert")
shelf.add(s, "Emma", "Austen")
print(shelf.add(s, "Dune", "Herbert"))   --> nil   already on the shelf: Dune
print(shelf.authors(s))                  --> Austen, Herbert
```

`ipairs` walks a sequence from 1 until the first nil, in order. `pairs` walks every key, but in no particular order, and the order can change between runs and Lua versions. Anything the player sees, such as the list of exits, needs an order you choose: walk a list you control, or collect the keys and `table.sort` them.

<!-- from:typescript -->
## Coming from TypeScript
- A table is an object, a `Map` and an array at once. `{ name = "Hall" }` is `{ name: "Hall" }`, and `t["two words"]` works like `obj["two words"]`.
- Lists start at 1, and `#list` is the length. `list[#list + 1] = x` is `push`.
- `for k, v in pairs(t)` is like `for (const [k, v] of Object.entries(t))`, except that the order is not insertion order. It's unspecified.
- A module returns its table: `return M` at the end does the job of `export`, and `local m = require("name")` the job of `import`.
<!-- /from -->

<!-- from:default -->
## Key points
- One table can be a record, a dictionary and a list. Pick the shape that makes lookups easy, and store the same table under two keys if you need both.
- `ipairs` is ordered; `pairs` is not. Sort, or walk a list you keep, when order matters.
- Expected failures return `nil, "message"`; callers check the first value.
<!-- /from -->

## Common mistake
Listing a dictionary with `pairs` and expecting the order you wrote it in:

```lua
local exits = { north = "kitchen", east = "garden" }
for direction in pairs(exits) do io.write(direction, " ") end
--> sometimes "north east", sometimes "east north"
```

Tests that compare such output pass on one machine and fail on the next. Walk `M.DIRECTIONS` instead.

## In this stage
In `world.lua`, implement `new`, `add_room`, `connect`, `exits`, `go`, `place`, `remove`, `describe` and `build`. A world is a plain table with `rooms` and `start`; items here are any tables with an `id` and a `name`. From the next stages on, the project folder keeps every file you write, and checking a stage runs the tests of every stage so far.