# Error context in Lua

Lua functions report expected failures by returning `nil` plus an error value, usually a string. Adding context is ordinary string work: check the second return value and return your own `nil, context .. ": " .. err`. Strings lose the original as a separate value; when callers need to inspect the cause, return a table instead (`{ message = ..., cause = err }`) and give it a `__tostring` metamethod so it still prints well. `error()` and `pcall` exist for raising, but for input that can be wrong, `nil, err` is the convention, and it's what the tests check.

```lua
local function read_port(env, name)
  local raw = env[name]
  if raw == nil then
    return nil, name .. ": missing"
  end
  local port, err = parse_port(raw)
  if port == nil then
    return nil, name .. ": " .. err
  end
  return port
end

print(select(2, read_port({ PORT = "http" }, "PORT")))  -- PORT: not a number: "http"
print(select(2, read_port({}, "PORT")))                 -- PORT: missing
```

<!-- from:typescript -->
## Coming from TypeScript
- No exceptions to catch: the error is the second return value, so wrapping is an `if` right after the call.
- A plain string has no `cause`; if you need one, return a table with a `cause` field.
- `..` is string concatenation; `nil .. "x"` raises an error, so check the value before building the message.
<!-- /from -->

<!-- from:default -->
## Key points
- `local value, err = f(...)` then `if value == nil then return nil, context .. ": " .. err end`.
- Put the context in front of the old message.
- Use a table error with a `cause` field when callers need the original.
<!-- /from -->

## Common mistake
Returning only the original error, or raising instead of returning:

```lua
if port == nil then return nil, err end   -- which variable was it?
if port == nil then error(err) end        -- the caller expected nil, err, not a raise
```

## In this exercise
`numbers.parse_number` returns `nil, reason` for bad input. `load_setting` finds the key, checks parse_number's result, and returns `nil, key .. ": " .. err` so the message mentions both.
