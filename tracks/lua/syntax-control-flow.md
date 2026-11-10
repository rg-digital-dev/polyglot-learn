# Blocks and Control Flow in Lua

Blocks end with keywords, not braces. `if` uses `then` and `end`, loops use `do` and `end`, and the extra branch word is spelled `elseif`. Indentation is only for readers. The numeric `for i = first, last, step` includes both ends. `for i, v in ipairs(xs)` walks a list in order, and `pairs(t)` visits every key in no promised order. `while` and `repeat ... until` cover the other cases; a `repeat` body can read its own locals in the `until` test. Lua has no `switch` and no `continue`. Use `if ... elseif` for choices, and `goto continue` with a `::continue::` label (or a wrapping `if`) to skip a round.

```lua
for i = 1, 3 do io.write(i, " ") end print()       --> 1 2 3
for i = 3, 1, -1 do io.write(i, " ") end print()   --> 3 2 1
for i, v in ipairs({"a", "b"}) do io.write(i, "=", v, " ") end print()
--> 1=a 2=b
for i = 1, 5 do
  if i % 2 == 0 then goto continue end
  io.write(i, " ")
  ::continue::
end
print()                                            --> 1 3 5
local n = 0
repeat n = n + 1 until n >= 3
print(n)                                           --> 3
```

<!-- from:python -->
## Coming from Python
- There is no `:` and no indentation rule. Write `then` or `do`, and close every block with `end`.
- `for i = 1, 3` includes 3. Python's `range(1, 3)` stops before it.
- `for x in xs` is `for _, x in ipairs(xs)` in Lua. The index comes first.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Conditions need no parentheses and no braces: `if x > 1 then ... end`.
- `switch` becomes an `if ... elseif` chain. There is no fall-through to guard against.
- `for (const x of xs)` is `for _, x in ipairs(xs)`, and the index starts at 1.
<!-- /from -->

<!-- from:default -->
## Key points
- Blocks are `then ... end` and `do ... end`; the second branch word is `elseif`.
- `for i = a, b` includes both `a` and `b`; a negative step counts down.
- `ipairs` walks `1..n` in order and stops at the first `nil`. `pairs` visits every key.
- There is no `switch` or `continue`. Use `elseif` and `goto continue`.
<!-- /from -->

## Common mistake
Looping from 0, as in other languages: `for i = 0, #xs do` reads `xs[0]`, which is `nil`. Write `for i = 1, #xs do`.
