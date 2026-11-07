# Error strategies in Lua

In Lua, a missing key is `nil`, and only `nil` and `false` are falsy, so `0` and `""` are real values. The usual trap is `tonumber(v) or fallback`: `tonumber` returns `nil` for anything it can't read, so a bad value silently becomes the fallback. `tonumber` is also lax in its own way. It accepts `" 7 "`, `"0x50"` (80) and `"1e3"` (the float `1000.0`). Match `^%d+$` first when only digits are allowed.

There's no "many errors" type. Collect the messages in a table and return them joined:

```lua
local problems = {}
if #name < 3 then
  problems[#problems + 1] = "name: too short"
end
if not email:find("@", 1, true) then
  problems[#problems + 1] = "email: missing @"
end
if #problems > 0 then
  return nil, table.concat(problems, "; ")
end
```

<!-- from:typescript -->
## Coming from TypeScript
- `t[key] == nil` is the presence check; tables have no inherited keys unless you set a metatable.
- `0` and `""` are truthy, but `tonumber(v) or fallback` still hides bad values.
- Errors are usually returned as `nil, message`, so "collect" means building a list of strings.
<!-- /from -->

<!-- from:default -->
## Key points
- `t[key] == nil` for "missing"; `0` and `""` are truthy.
- `tonumber` returns `nil` for bad input, and accepts hex and exponents.
- `table.concat(problems, "; ")` joins collected messages.
<!-- /from -->

## Common mistake
Letting `or` pick the fallback:

```lua
return tonumber(settings[key]) or fallback -- "eighty" and "" both give the fallback
```

## In these exercises
`int_setting` returns the fallback only when the value is `nil`, matches `^%d+$`, and returns `nil, key .. ": not a number…"` otherwise. `validate_signup` collects the problems and returns `nil, table.concat(problems, "; ")`.
