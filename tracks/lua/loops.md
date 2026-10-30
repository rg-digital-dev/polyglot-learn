# Loops in Lua

Lua has a numeric `for i = start, stop, step do ... end`, whose bounds are both inclusive, and a generic `for` that walks an iterator: `for i, v in ipairs(list)` visits a sequence in order from index 1, and `for k, v in pairs(t)` visits every key. There are also `while cond do ... end` and `repeat ... until cond`. `break` leaves a loop early. Lua has no `continue`; you restructure the body or jump to a label with `goto`.

```lua
-- Product of the even numbers from 2 to n, with an early exit on overflow risk.
local function even_product(n)
  local product = 1
  for i = 2, n, 2 do          -- 2, 4, 6, ... up to and including n
    product = product * i
    if product > 1e12 then break end
  end
  return product
end

-- Index of the first negative value, or nil.
local function first_negative(values)
  for i, v in ipairs(values) do
    if v < 0 then return i end
  end
  return nil
end

print(even_product(8))                 --> 384
print(first_negative({4, 0, -2, 7}))   --> 3
```

<!-- from:typescript -->
## Coming from TypeScript
- `for i = 1, n` includes `n`. The TS loop `for (let i = 0; i < n; i++)` becomes `for i = 0, n - 1`.
- Lists start at index 1, so `ipairs` yields `1, 2, 3, ...`, not `0, 1, 2`.
- `ipairs` is like `for...of` with an index; `pairs` is like `for...in` over an object's keys.
- There is no `continue`. Nest the body in an `if`, or use `goto continue` with a `::continue::` label before `end`.
- The loop variable is local to the loop; assigning to it does not change the iteration.
<!-- /from -->

<!-- from:default -->
## Key points
- Numeric `for` bounds are inclusive at both ends and evaluated once.
- `ipairs(t)` walks `t[1], t[2], ...` and stops at the first `nil`.
- `break` exits the innermost loop; a `return` inside a loop exits the function.
- An empty list simply runs the loop body zero times.
<!-- /from -->

## Common mistake
Forgetting that the upper bound is included:

```lua
for i = 1, limit do ... end      -- visits limit itself
for i = 1, limit - 1 do ... end  -- fixed when you need "below limit"
```

## In this exercise
Implement `function M.sum_of_multiples(factors, limit)` in `sum-of-multiples/sum_of_multiples.lua`. `factors` is a sequence of integers (possibly empty) and the result is an integer. Note that `n % 0` with integers raises `attempt to perform 'n%%0'`, so a factor of 0 has to be skipped before you use it.
