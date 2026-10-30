# Conditionals in Lua

Lua branches with `if ... then ... elseif ... then ... else ... end`. There is no `switch`. Only two values are false: `false` and `nil`. Everything else is true, including `0` and the empty string. The logical operators are the words `and`, `or` and `not`, inequality is `~=`, and `and`/`or` return one of their operands rather than a strict boolean. In Lua 5.4, `%` on two integers gives an integer, while `/` always gives a float.

```lua
local function shipping_cost(weight_kg, express)
  if weight_kg <= 0 then
    return nil
  elseif weight_kg < 2 then
    return express and 12 or 5
  else
    return (express and 20 or 9) + (weight_kg - 2) * 2
  end
end

print(shipping_cost(1, false))  --> 5
print(shipping_cost(3, true))   --> 22
print(10 % 3, 10 / 2, 10 // 3)  --> 1   5.0   3
print(0 and "zero is true")     --> zero is true
```

<!-- from:typescript -->
## Coming from TypeScript
- `0`, `""` and `NaN` are truthy in Lua. Only `nil` and `false` are falsy.
- Write `~=` for "not equal". There is no `!=`, and no `===` either: `==` never converts types, so `1 == "1"` is `false`.
- `&&`, `||` and `!` are `and`, `or` and `not`.
- `cond and a or b` works like `cond ? a : b`, except when `a` is `false` or `nil`.
- `10 / 2` is `5.0`, a float. Use `//` for floor division and `%` for remainders.
<!-- /from -->

<!-- from:default -->
## Key points
- Only `false` and `nil` count as false in a condition.
- `and` and `or` short-circuit and return an operand, which makes `x or default` a common idiom.
- Comparison operators: `==`, `~=`, `<`, `<=`, `>`, `>=`.
- `a % b` is the remainder; with integer operands the result is an integer.
<!-- /from -->

## Common mistake
Testing a remainder as if `0` were false:

```lua
if year % 4 then ... end       -- always runs: 0 is a true value
if year % 4 == 0 then ... end  -- fixed: compare explicitly
```

## In this exercise
Implement `function M.leap_year(year)` in `leap-year/leap_year.lua`. `year` is an integer and the tests compare against the booleans `true` and `false`, so return a real boolean (comparisons combined with `and`/`or` already give one).
