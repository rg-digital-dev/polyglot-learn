# Maps in Lua

Lua has one data structure, the table, and a map is simply a table with non-integer keys. `t[key] = value` inserts or updates, reading a missing key gives `nil`, and assigning `nil` removes the key. `t.name` is sugar for `t["name"]`, so it only works with a literal key. `pairs(t)` visits every key/value pair, but the order is unspecified and can change between runs. Because a missing key reads as `nil`, the "get or default" idiom is `(t[k] or 0) + 1`.

```lua
-- Count file extensions in a list of paths.
local function count_extensions(paths)
  local counts = {}
  for _, path in ipairs(paths) do
    local ext = path:match("%.(%w+)$") or "(none)"
    counts[ext] = (counts[ext] or 0) + 1
  end
  return counts
end

local counts = count_extensions({"a.lua", "b.txt", "c.lua", "Makefile"})
print(counts.lua, counts["txt"], counts.md)  --> 2   1   nil

local keys = {}
for ext in pairs(counts) do keys[#keys + 1] = ext end
table.sort(keys)                             -- sort keys when order matters
print(table.concat(keys, ","))               --> (none),lua,txt
```

<!-- from:typescript -->
## Coming from TypeScript
- A table is both `Map` and `Array`; there is no separate `Map` class.
- `t.key` is like `obj.key` and `t[key]` like `obj[key]`, but keys keep their type: `t[1]` and `t["1"]` are different entries.
- A missing key is `nil`, never an exception. `t[k] or 0` plays the role of `?? 0`.
- `#t` does not count map entries. Count with a `pairs` loop if you need the size.
- `pairs` order is unspecified, unlike JS objects and `Map`, which keep insertion order.
<!-- /from -->

<!-- from:default -->
## Key points
- Insert with `t[k] = v`, delete with `t[k] = nil`.
- `(t[k] or 0) + 1` increments a counter that may not exist yet.
- Iterate with `pairs(t)` and never rely on its order.
- `string.lower` returns a lowercased copy; strings are never changed in place.
<!-- /from -->

## Common mistake
Writing `t.word` when the key is in a variable:

```lua
local word = "fish"
counts.word = (counts.word or 0) + 1      -- counts the literal key "word"
counts[word] = (counts[word] or 0) + 1    -- fixed: uses the value of `word`
```

## In this exercise
Implement `function M.word_count(text)` in `word-count/word_count.lua`. Return a table whose keys are lowercase words and whose values are integer counts; empty text gives `{}`. The tests compare tables key by key, so order doesn't matter.
