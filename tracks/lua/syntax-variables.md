# Variables and Constants in Lua

You declare a variable with `local`: `local count = 1`. It lives until the end of the enclosing block (`do ... end`, a function body, an `if` branch or a loop body). Assigning to a name that has no `local` does not fail: it creates a global that every part of the program can see. Lua 5.4 adds `<const>`: `local MAX <const> = 3` cannot be reassigned, and trying to is an error when the file loads, before any code runs. `<const>` guards the name only: a table in a const variable can still change. You can assign several names at once, which makes a swap one line.

```lua
local count = 1
count = count + 1
local MAX <const> = 3
local xs <const> = {1, 2}
xs[#xs + 1] = 3
print(count, MAX, #xs)  --> 2   3   3
local function f() total = 5 end  -- no 'local': a global
f()
print(total)            --> 5
local a, b = 1, 2
a, b = b, a
print(a, b)             --> 2   1
print(load("local K <const> = 1; K = 2"))
--> nil   [string "..."]:1: attempt to assign to const variable 'K'
```

<!-- from:python -->
## Coming from Python
- Plain assignment makes a global in Lua, not a local. Python's rule is the other way round, so add `local` every time.
- A new block is a new scope. A `local` made inside an `if` or a loop is gone after its `end`.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- `local` is closest to `let`. `<const>` is closest to `const`, and needs Lua 5.4.
- A misspelled name is not an error: reading an unknown name gives `nil`, and assigning one makes a global.
<!-- /from -->

<!-- from:default -->
## Key points
- Always write `local` unless you really want a global.
- `local NAME <const> = value` (Lua 5.4) stops reassignment and is checked at load time.
- `<const>` does not freeze a table's contents.
- `local a, b = 1, 2` and `a, b = b, a` assign several names at once.
<!-- /from -->

## Common mistake
Leaving out `local` inside a function. The assignment works, so nothing complains, but the variable is now global and can clash with another function's variable of the same name.
