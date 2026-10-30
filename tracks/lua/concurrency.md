# Concurrency in Lua

Standard Lua has no threads, no event loop and no `sleep`. What it has is coroutines: functions that can pause themselves with `coroutine.yield` and continue later, where they left off, when someone calls `coroutine.resume`. Only one coroutine runs at a time, and it keeps running until it yields, so this is cooperative multitasking. To run several tasks "at once" you write a small scheduler: a loop that resumes each unfinished coroutine in turn. That is why this exercise uses a virtual clock instead of real time. Each task yields once per tick, the scheduler advances the clock once per round, and the whole run takes as many ticks as the longest task. Real hosts use the same idea. FiveM's `Citizen.CreateThread` starts your function as a Lua coroutine, and `Wait(ms)` yields it back to FiveM's scheduler until the time is up. Neovim instead offers callback-based timers and I/O through `vim.uv` (libuv; `vim.loop` is the older, deprecated name).

```lua
local co = coroutine.create(function(name, n)
  for i = 1, n do coroutine.yield(name .. " " .. i .. "/" .. n) end
  return name .. " done"
end)

print(coroutine.resume(co, "a.iso", 2))  --> true   a.iso 1/2   (the first resume passes the arguments)
print(coroutine.resume(co))              --> true   a.iso 2/2
print(coroutine.resume(co))              --> true   a.iso done  (the return value)
print(coroutine.status(co))              --> dead
print(coroutine.resume(co))              --> false  cannot resume dead coroutine

-- wrap gives a plain function that resumes the coroutine on each call.
local next_id = coroutine.wrap(function()
  local id = 0
  while true do id = id + 1; coroutine.yield(id) end
end)
print(next_id(), next_id())              --> 1   2
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `Promise`, `setTimeout` or built-in event loop; you resume coroutines yourself.
- `yield` is a little like `await`, but it works from any function the coroutine calls, not only inside an `async` function.
- As in JavaScript, nothing runs in parallel: this is concurrency on one thread.
- `coroutine.resume` never throws. It returns `true` plus the yielded or returned values, or `false` plus an error message.
<!-- /from -->

<!-- from:default -->
## Key points
- `coroutine.create(f)` makes a paused coroutine; `coroutine.resume` runs it until the next `yield` or the end.
- `coroutine.status(co)` is `"suspended"` while it can continue and `"dead"` when it has finished.
- The final `resume` returns the function's return values.
- A round-robin loop that resumes every unfinished coroutine once per round runs them side by side.
<!-- /from -->

## Common mistake
Calling a yielding function directly instead of inside a coroutine:

```lua
local result = M.task(100)                 -- attempt to yield from outside a coroutine
local co = coroutine.create(M.task)        -- fixed: wrap it, then resume it
```

## In this exercise
Implement `function M.run_all(delays)` in `run-all/run_all.lua`. The stub provides `M.clock`, `M.tick()` and `M.task(ms)`; don't change them. Create one coroutine per delay, then resume every unfinished one in turn, calling `M.tick()` once per round, and store each task's result at its index. The tests check the results and that the virtual clock advanced by about the largest delay, not the sum.
