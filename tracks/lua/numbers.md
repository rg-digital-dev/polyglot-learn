# Numbers in Lua

Lua 5.4 numbers have two subtypes, 64-bit integers and floats, and `math.type(x)` tells them apart. `/` always gives a float (`6 / 2` is `3.0`). `//` is floor division and `%` its matching modulo, so the remainder takes the divisor's sign; on integers both stay integers. Integer overflow wraps around silently: `math.maxinteger + 1 == math.mininteger`. There is no `math.round`; `math.floor(x + 0.5)` rounds halves up, toward positive infinity. An integer and a float compare equal when they have the same value (`3 == 3.0`).

```lua
print(-7 // 2, -7 % 2, -7 / 2)          --> -4  1  -3.5
print(math.type(7 // 2), math.type(6 / 2))  --> integer  float
print(math.floor(2.5 + 0.5), math.floor(-2.5 + 0.5))  --> 3  -2
print(math.maxinteger + 1 == math.mininteger)  --> true
print(pcall(function() return 7 // 0 end))     --> false  ...attempt to divide by zero
```

<!-- from:typescript -->
## Coming from TypeScript
- `//` is `Math.floor(a / b)` with exact integers; `%` floors too, so `-1 % 5` is `4`.
- Integers are exact all the way to 2^63 − 1, and then wrap without an error.
- Integer division by zero is an error, not `Infinity` (float division still gives `inf`).
<!-- /from -->

<!-- from:default -->
## Key points
- `//` and `%` floor, for integers and floats alike.
- `/` always returns a float.
- Integer overflow wraps silently; `math.maxinteger` and `math.mininteger` are the limits.
- There is no built-in round; you choose the rule.
<!-- /from -->

## Common mistake
Rounding negative halves with `floor(x + 0.5)`:

```lua
local function round(x) return math.floor(x + 0.5) end
print(round(12.5), round(-12.5))   --> 13  -12, not -13
```

## In these exercises
`wrap_index` and `split_evenly` are easy in Lua, because `//` and `%` already floor. `round_percent` needs halves rounded away from zero, which `floor(x + 0.5)` gets wrong for negative numbers. `checked_multiply` has to notice a wrapped product, for example by dividing it back.
