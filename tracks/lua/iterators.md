# Iterators in Lua

The generic `for` works with any iterator, not just `pairs` and `ipairs`. `for a, b in f, s, c do` calls `f(s, c)` again and again, putting the first result into the next call as `c`, and stops when that first result is `nil`. A stateless iterator keeps nothing between calls; it computes the next step from `s` and `c`, as `ipairs` does. A stateful iterator is a closure that remembers its position in an upvalue and ignores the arguments. `string.gmatch` is a built-in stateful iterator over pattern matches. Lua has no `map`/`filter`/`sort` chain on tables, so a pipeline is a few short steps: loop to collect, `table.sort`, then loop again to take what you need.

```lua
-- Stateful: a closure that returns the next value, or nil when it is done.
local function chunks(list, size)
  local i = 0
  return function()
    if i >= #list then return nil end
    local chunk = table.move(list, i + 1, math.min(i + size, #list), 1, {})
    i = i + size
    return chunk
  end
end
for c in chunks({ "a", "b", "c", "d", "e" }, 2) do print(table.concat(c)) end
                                          --> ab, then cd, then e

-- Stateless: the generic for passes the state and the last index back in.
local function step2(list, i)
  i = i + 2
  if list[i] ~= nil then return i, list[i] end
end
local function every_other(list) return step2, list, -1 end
for i, v in every_other({ "a", "b", "c", "d", "e" }) do print(i, v) end
                                          --> 1 a, then 3 c, then 5 e

for key, value in ("w=1920 h=1080"):gmatch("(%a+)=(%d+)") do print(key, value) end
                                          --> w 1920, then h 1080
```

<!-- from:typescript -->
## Coming from TypeScript
- The protocol is a function returning values until the first is `nil`, not an object with `next()` returning `{ value, done }`.
- `gmatch` is roughly `matchAll`, and it yields the captures directly.
- There are no array methods. Write the loop, or a small helper such as `map(t, f)` if you need it often.
- `pairs` order is unspecified, so collect entries into a sequence and sort it before you take the first n.
<!-- /from -->

<!-- from:default -->
## Key points
- `for ... in` calls the iterator function until it returns `nil`.
- Stateless iterators get their state as arguments; stateful ones keep it in a closure.
- `string.gmatch(s, pattern)` iterates over matches; with captures it yields each capture.
- To rank map entries: copy them into a sequence of `{ key, value }` records, then sort.
<!-- /from -->

## Common mistake
Looping over a table without an iterator:

```lua
for word, n in counts do end          -- attempt to call a table value (for iterator 'for iterator')
for word, n in pairs(counts) do end   -- fixed
```

## In this exercise
Implement `function M.top_words(text, n)` in `top-words/top_words.lua`. Return a sequence of at most `n` lowercase words, most frequent first, with ties in alphabetical order. Every word appears once in the counts, so a comparator that checks count and then word fully decides the order and the sort's instability doesn't matter.
