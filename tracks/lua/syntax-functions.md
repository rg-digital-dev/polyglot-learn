# Functions in Lua

A function is a value. `local function add(a, b) return a + b end` is a private function, and `function M.add(a, b) ... end` stores one in a table field. There are no types and no default values in the signature. Missing arguments arrive as `nil`, extra arguments are dropped, and `b = b or 10` in the body fills a default. A function can return several values: `return q, r`. The caller takes them with `local q, r = f()`. A function without a name is `function(x) return x * x end`. A call in the middle of an argument list keeps only its first result, while a call at the end passes on all of them. `...` collects extra arguments.

```lua
local function add(a, b) return a + b end
local square = function(x) return x * x end
print(add(1, 2), square(4))        --> 3   16
print(add(1, 2, 99))               --> 3
print(pcall(add, 1))
--> false   ...: attempt to perform arithmetic on a nil value (local 'b')
local function opt(a, b) b = b or 10 return a + b end
print(opt(1), opt(1, 2))           --> 11   3
local function divmod(a, b) return a // b, a % b end
local q, r = divmod(7, 2)
print(q, r)                        --> 3   1
local function pair() return 1, 2 end
print(pair(), (pair()))            --> 1   1
```

<!-- from:python -->
## Coming from Python
- No `def`, no `:` and no keyword arguments. Write `function name(a, b) ... end`.
- Defaults do not exist. Use `b = b or 1`, which also replaces an explicit `false`.
- `return a, b` is real multiple return, not a tuple. `print(pair())` shows both values.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- There are no arrow functions: use `function(a, b) return a + b end`.
- Calling with too few arguments is not an error until a `nil` is used.
- `return q, r` replaces returning an array. Take both with `local q, r = f()`, no destructuring needed.
<!-- /from -->

<!-- from:default -->
## Key points
- Functions are values. `local function f` is private, and `function M.f` joins the module.
- Missing arguments are `nil`, and extra ones are dropped.
- `return a, b` returns two values, and `(f())` keeps only the first.
- The result is what the function `return`s; a function with no `return` gives back nothing.
<!-- /from -->

## Common mistake
Printing instead of returning. `print(total)` shows the right number in your terminal and the function still returns nothing, so the test sees `nil`.
