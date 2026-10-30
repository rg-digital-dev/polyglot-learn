# Errors in Lua

Lua has two ways to report failure. For expected failures, such as bad input or a missing file, a function returns `nil` followed by an error message; the standard library works this way (`io.open` returns `nil, "x: No such file or directory"`), and callers check the first value. For bugs and truly exceptional cases, `error(msg)` raises an error that unwinds the stack until something catches it with `pcall(f, ...)`. `assert(v, msg)` bridges the two: it returns `v` when it is truthy and raises `msg` otherwise.

```lua
-- Parse "WIDTHxHEIGHT" such as "1920x1080".
local function parse_size(input)
  local w, h = input:match("^(%d+)x(%d+)$")
  if not w then
    return nil, "expected WIDTHxHEIGHT, got " .. string.format("%q", input)
  end
  w, h = math.tointeger(tonumber(w)), math.tointeger(tonumber(h))
  if not w or not h or w == 0 or h == 0 then
    return nil, "size out of range"
  end
  return w, h
end

local w, h = parse_size("1920x1080")   --> 1920   1080
local ok, err = parse_size("big")      --> nil    expected WIDTHxHEIGHT, got "big"
if not ok then print("bad size: " .. err) end
```

<!-- from:typescript -->
## Coming from TypeScript
- The usual style is `return nil, "message"`, not `throw`. The caller writes `local v, err = f()` and checks `if not v`.
- `error()` and `pcall()` exist and act like `throw` and `try/catch`, but they are for exceptional cases.
- `tonumber(s)` returns `nil` instead of `NaN` when parsing fails, so check for `nil`.
- `tonumber` accepts more than plain digits: `"+80"`, `" 80 "`, `"0x50"` and `"1e2"` all parse.
- A digit string too large for a 64-bit integer comes back as a float (`1e+20`), not an error. `math.type(n)` tells you which you got.
<!-- /from -->

<!-- from:default -->
## Key points
- Return `value` on success and `nil, message` on failure.
- Callers check the first return value before using it.
- Use `error` for programming mistakes, and `pcall` to catch raised errors.
- `tonumber` returns `nil` on failure, and returns a float when an integer would overflow.
<!-- /from -->

## Common mistake
Raising when the contract says "return nil, err":

```lua
if n > 65535 then error("out of range") end        -- the caller's `local v, err = f()` blows up
if n > 65535 then return nil, "out of range" end   -- fixed
```

## In this exercise
Implement `function M.parse_port(input)` in `parse-port/parse_port.lua`. Return the port as an integer, or `nil, "message"` for invalid input. A raised error counts as a failure in the tests, even for inputs that should be rejected.
