# Reading files in Lua

`io.open(path, mode)` opens a file and returns a file handle, or `nil` plus a message that includes the path when it can't. `file:read("a")` reads everything that's left as a string, and `file:close()` releases the handle; close it as soon as you're done. For line-by-line reading, `for line in file:lines() do ... end` (or `io.lines(path)`, which raises on error instead). Open with `"rb"` to read the bytes as they are on every system.

```lua
local function first_line(path)
  local file, err = io.open(path, "rb")
  if not file then
    return nil, err -- "does-not-exist.txt: No such file or directory"
  end
  local line = file:read("l") or ""
  file:close()
  return (line:gsub("%s+$", ""))
end

print(first_line("does-not-exist.txt"))
```

<!-- from:typescript -->
## Coming from TypeScript
- No exception: `io.open` returns `nil, message`, the usual Lua convention.
- The handle must be closed with `file:close()`; there's no garbage-collected close you should rely on.
- Patterns aren't regular expressions: `%s` is whitespace, and `gsub` returns two values, so wrap it in parentheses to keep one.
<!-- /from -->

<!-- from:default -->
## Key points
- `local file, err = io.open(path, "rb")`, then check `file`.
- `file:read("a")` reads everything; `file:close()` when done.
- Trim each line with a pattern like `^%s*(.-)%s*$`, which also removes `\r`.
<!-- /from -->

## Common mistake
Calling a method on the `nil` handle:

```lua
local text = io.open(path):read("a") -- attempt to index a nil value: the reason is lost
```

## In this exercise
`read_lines(path)` reads the file, returns `nil` plus a message naming it if it can't, and otherwise the trimmed lines that aren't blank or `#` comments. The tests pass the path of files `learn` puts next to them.
