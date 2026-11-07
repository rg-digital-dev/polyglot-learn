# Text in Lua

Lua strings are byte strings. `#s`, `s:sub(i, j)`, `s:byte(i)` and string patterns all work in bytes; a pattern's `.` matches one byte. Lua 5.3 added the `utf8` library for UTF-8 text: `utf8.len(s)` counts code points, `for p, cp in utf8.codes(s)` gives each code point's byte position and number, `utf8.char(cp)` turns a number back into text, and `utf8.offset(s, n)` gives the byte position where the `n`th code point starts. `utf8.charpattern` matches exactly one UTF-8 character.

```lua
local s = "Zoë 😀"
print(#s, utf8.len(s))            -- 9  5
for p, cp in utf8.codes(s) do
  io.write(p, ":", utf8.char(cp), " ")   -- 1:Z 2:o 3:ë 5:  6:😀
end
print()
print(s:sub(1, utf8.offset(s, 4) - 1))   -- Zoë: everything before the 4th code point
```

<!-- from:typescript -->
## Coming from TypeScript
- `#s` counts bytes, not UTF-16 units: `#"é"` is 2.
- `utf8.codes` gives numbers; `s:gmatch(utf8.charpattern)` gives one-character strings, like `for (const ch of s)`.
- Strings are 1-indexed, and `sub(i, j)` includes `j`.
<!-- /from -->

<!-- from:default -->
## Key points
- `#`, `sub` and patterns work in bytes.
- `utf8.len`, `utf8.codes`, `utf8.offset` and `utf8.charpattern` work in code points.
- `utf8.len` returns `nil` (plus a position) for invalid UTF-8.
<!-- /from -->

## Common mistake
Using a character count as a byte position:

```lua
local s = "résumé"
print(s:sub(1, 2))                        -- "r" and half of é
print(s:sub(1, utf8.offset(s, 3) - 1))    -- "ré"
```

## In these exercises
`char_count(text)` returns a table from each code point (as a string) to its count: loop with `text:gmatch(utf8.charpattern)`. `truncate(text, limit)` counts with `utf8.len` and cuts with `utf8.offset`.
