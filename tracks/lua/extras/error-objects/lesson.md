# Error objects, xpcall and rethrowing

`error(v)` can raise any value, not just a string. Raising a table such as `{ code = "bad_port", text = "..." }` gives the catcher something it can inspect, which is much more reliable than matching on message text. `pcall(f, ...)` returns `true` plus f's results, or `false` plus the raised value, the very same table. By the time `pcall` returns, the stack is gone. `xpcall(f, handler, ...)` calls `handler(err)` first, while the failing frames still exist, so `debug.traceback()` inside the handler records where the error happened; whatever the handler returns is what `xpcall` returns. For string errors, `error(msg, level)` chooses whose position is put in front of the message: level 1 (the default) is the function that called `error`, level 2 is the function that called it, and level 0 adds no position at all. To rethrow an error you caught, call `error` again: build a new table to add context, or pass the old value with level 0 so it doesn't get a second position. Finally, `assert(v, err)` converts the "return `nil, err`" style into a raised error: it returns its arguments when `v` is truthy and raises `err` otherwise.

```lua
-- Saved as example.lua: the positions in the output come from that name.
local function parse_port(s)
  local n = math.tointeger(tonumber(s))
  if not n then error({ code = "bad_port", text = "not a number: " .. s }) end
  return n
end

-- pcall returns false plus whatever was raised: here, the table itself.
local ok, e = pcall(parse_port, "http")
print(ok, e.code, e.text)            --> false  bad_port  not a number: http

-- xpcall calls the handler before the stack unwinds, so the traceback
-- still shows where the error came from.
local ok2, e2 = xpcall(parse_port, function(err)
  err.trace = debug.traceback("", 2)
  return err
end, "ftp")
print(ok2, e2.trace:match("example.lua:%d+"))  --> false  example.lua:4 (the error call)

-- error's second argument picks whose position goes in front of a string.
local function need(v, what)
  if v == nil then error(what .. " is required", 2) end  -- 2 = our caller
  return v
end
local function connect(host)
  local h = need(host, "host")       -- the message points at this line
  return h
end
print(pcall(connect))                --> false  example.lua:26: host is required

-- assert turns a `nil, err` return into a raised error.
print(pcall(function() return assert(io.open("missing.txt")) end))
--> false  example.lua:32: missing.txt: No such file or directory
```

<!-- from:typescript -->
## Coming from TypeScript
- `throw { code, message }` and `catch (e) { e.code }` map to `error({ code = ..., message = ... })` and `local ok, e = pcall(f); if not ok then ... e.code`.
- There is no `Error` class with a built-in `.stack`. If you want a stack trace, take it yourself with `debug.traceback()` in an `xpcall` handler.
- Rethrowing is just `error(e, 0)` inside the failure branch; there is no `finally`, so clean up before you rethrow.
- `pcall` is a function call, not a block: wrap the code in a function and pass its arguments after it.
<!-- /from -->

<!-- from:default -->
## Key points
- Raise tables with a `code` field when callers need to tell errors apart.
- Use `xpcall` with a handler when you need a traceback; `pcall` loses the stack.
- `error(msg, 2)` blames the caller. It finds no caller line when the call was a tail call (`return f(...)`).
- `assert(v, err)` turns `nil, err` into a raised error; `pcall` goes the other way.
<!-- /from -->

## Common mistake
Letting a table error reach the top level. The standalone interpreter can only print strings, so all you see is:

```lua
error({ code = "bad_port" })
-- lua: (error object is a table value)

-- fixed: catch it with pcall/xpcall, or give the table a __tostring metamethod
error(setmetatable({ code = "bad_port" }, { __tostring = function(e) return "error " .. e.code end }))
-- lua: error bad_port
```

## In this exercise
Implement five functions in `error-objects/error_objects.lua`. `check_age` raises error objects with a `code` and a `message`. `try` catches with `xpcall`, returns `nil, err`, wraps string errors as `{ code = "internal", ... }` and attaches a traceback. `must` turns that back into a raised error. `require_field` raises a string that names the caller's line, and `with_context` rethrows error objects with extra context and other errors unchanged. The tests make sure the stub's own `TODO: implement` error never counts as the expected error.
