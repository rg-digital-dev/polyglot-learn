# Cleanup in Lua

Lua 5.4 has *to-be-closed* variables: `local f <close> = value` calls the value's `__close` metamethod when the variable goes out of scope, by reaching the end of the block, by `return`, `break`, or an error. Values without `__close` can't be declared that way (except `nil` and `false`, which are ignored). It covers one variable at a time, so for a list of resources of unknown length you keep them in a table and close them yourself, on every path that leaves the function, most easily with a small `close_all` helper.

```lua
local function first_lines(paths)
  local files = {}
  local function close_all()
    for i = #files, 1, -1 do files[i]:close() end
  end
  for _, path in ipairs(paths) do
    local f, err = io.open(path)
    if not f then
      close_all()
      return nil, err
    end
    files[#files + 1] = f
  end
  local lines = {}
  for i, f in ipairs(files) do lines[i] = f:read("l") end
  close_all()
  return lines
end
```

<!-- from:typescript -->
## Coming from TypeScript
- `<close>` is Lua's `using`: one variable, closed at the end of its block.
- There's no `DisposableStack`: a table and a `close_all` helper do the job.
- Errors are returned, not thrown, so each early `return nil, err` needs the cleanup too.
<!-- /from -->

<!-- from:default -->
## Key points
- `local r <close> = open(...)` for a single resource.
- For several, keep a table and close it before every `return`.
- `nil` is allowed in a `<close>` variable, so check for errors after the declaration.
<!-- /from -->

## Common mistake
Remembering the cleanup on the normal path and forgetting an error path:

```lua
local data, err = r:read()
if not data then
  return nil, err -- everything opened before is still open
end
```

## In this exercise
`resource.open(name)` (from `resource.lua`) returns a resource with `read()`, `close()` and `__close`, or `nil` plus an error. `read_all` opens them all into a table, reads each, and closes the table on all three ways out.
