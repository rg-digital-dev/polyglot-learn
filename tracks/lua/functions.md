# Functions and strings in Lua

Functions in Lua are values: `function M.name(a, b) ... end` stores a function in the field `name` of the table `M`, which is how modules expose their API. Missing arguments arrive as `nil`, extra ones are dropped, and a function can return several values at once. Strings are immutable byte sequences. You join them with `..`, and the `string` library (`string.format`, `string.upper`, `string.match`, ...) can also be called with method syntax: `s:upper()` means `string.upper(s)`.

```lua
local M = {}

-- `unit or "pcs"` supplies a default when the caller passes nothing.
function M.label(item, unit)
  unit = unit or "pcs"
  return string.format("%s (%s)", item, unit)
end

-- Two return values; string.match returns one value per capture.
function M.split_key(pair)
  local key, value = pair:match("^(%w+)=(.*)$")
  return key, value
end

print(M.label("apples"))          --> apples (pcs)
print(M.split_key("color=blue"))  --> color   blue
print(("ab"):rep(3, "-"))         --> ab-ab-ab
print(#"Zoë")                     --> 4 (bytes, not characters)
```

<!-- from:typescript -->
## Coming from TypeScript
- Concatenation is `..`, not `+`. `"a" + "b"` raises an arithmetic error.
- There are no template literals; `string.format("%s, %d", s, n)` fills the gap.
- `s:len()` and `#s` count bytes, not UTF-16 units or characters. `"Zoë"` has length 4.
- Default parameters don't exist; write `x = x or default` at the top of the body.
- `local` matters: a variable without it is global, the opposite of `let`/`const`.
- Patterns in `string.match`/`gsub` look like regexes but are smaller: `%s` is whitespace, `%d` a digit, `-` a lazy repeat, and there is no `|`.
<!-- /from -->

<!-- from:default -->
## Key points
- A module is a table: `local M = {}`, add `function M.f(...)`, then `return M`.
- Join strings with `..` or build them with `string.format`.
- `s:method()` is shorthand for `string.method(s)`.
- String functions work on bytes, so non-ASCII text passes through untouched.
<!-- /from -->

## Common mistake
Using `+` to build a string:

```lua
return "Hello, " + name                    -- error: attempt to add a 'string' with a 'string'
return "Hello, " .. name .. "!"            -- fixed
return string.format("Hello, %s!", name)   -- also fine
```

## In this exercise
Implement `function M.greet(name)` in `hello/hello.lua` and return the greeting string. Trimming only has to handle ASCII whitespace, which is exactly what the `%s` pattern class matches.
