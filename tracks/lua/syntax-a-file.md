# A File at a Glance in Lua

A Lua exercise file is a module. It starts with `local M = {}`, adds each function as a field of that table (`function M.greet(name) ... end`), and ends with `return M`. The tests load the file with `require("hello")` and call `M.greet`, so the table is what makes your function visible from outside. Comments start with `--` and run to the end of the line; `--[[ ... ]]` spans several lines. The `---` lines above a function are ordinary comments that editors read as documentation (`--- @param name string`). Statements end at the end of the line, and `;` is optional. `print` writes its arguments separated by tabs.

```lua
-- A comment runs to the end of the line.
local M = {}

--- Return a greeting.
--- @param name string
--- @return string
function M.greet(name)
  return "Hello, " .. name .. "!"
end

--[[ a block comment
     over several lines ]]
print(M.greet("Alice"))  --> Hello, Alice!
local a = 1; local b = 2  -- ';' is optional
print(a + b)              --> 3
return M
```

<!-- from:python -->
## Coming from Python
- There is no `def` and no indentation rule. Blocks end with the keyword `end`, so you must write it.
- Nothing is exported by default. A function that is not stored in `M` cannot be reached by the tests.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- `return M` at the bottom plays the role of `export`. Forgetting it makes `require` give back `true` instead of your table.
- `--` is the comment marker. `//` is floor division in Lua 5.3 and later, not a comment.
<!-- /from -->

<!-- from:default -->
## Key points
- Put your function in the table: `function M.name(...) ... end`. Keep the name from the stub.
- The file must end with `return M`.
- `--` starts a comment; `--[[ ... ]]` is a block comment.
- Use `print(x)` to look at a value, and remove it before you finish if it is noisy.
<!-- /from -->

## Common mistake
Writing `local function greet(name) ... end` instead of `function M.greet(name) ... end`. A `local` function is private to the file, so the tests report that `M.greet` is `nil`.
