local M = {}

-- A generator here is a function you call again and again to get the next
-- value; it returns nil once it has no more values.

--- A generator for from, from + step, ... up to and including `to` (down to
--- `to` for a negative step), usable as `for i in M.range(1, 5) do`.
--- `step` defaults to 1. Build it with coroutine.wrap.
--- @param from integer
--- @param to integer
--- @param step integer|nil
--- @return (fun(): integer|nil)|nil gen
--- @return string|nil err  return nil, "message" if step is 0; don't raise
function M.range(from, to, step)
  error("TODO: implement range")
end

--- An endless generator: 1, 2, 3, ...
--- @return fun(): integer
function M.naturals()
  error("TODO: implement naturals")
end

--- The first n values of `gen` as a list. Call gen at most n times, and stop
--- early when it returns nil.
--- @param gen fun(): any
--- @param n integer
--- @return any[]
function M.take(gen, n)
  error("TODO: implement take")
end

--- A coroutine (made with coroutine.create) that keeps a running total.
--- Each `coroutine.resume(co, n)` adds n and yields the total so far.
--- Resuming with no value ends the coroutine, which returns the final total.
--- @return thread
function M.running_total()
  error("TODO: implement running_total")
end

return M
