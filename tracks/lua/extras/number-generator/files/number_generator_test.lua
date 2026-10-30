-- Part of the Deeper Lua module (tracks/lua/extras/number-generator). Don't
-- edit this file: `learn` overwrites it. Your code goes in number_generator.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local M = require("number_generator")

local t = H.new()

-- Run a generic for over the generator and collect what it produces.
local function collect(gen)
  if type(gen) ~= "function" then error("expected a generator function, got " .. type(gen), 0) end
  local out = {}
  for v in gen do
    out[#out + 1] = v
    if #out > 1000 then error("the generator never ended", 0) end
  end
  return out
end

t:equals("c01_range_counts_up_inclusive", function() return collect(M.range(1, 5)) end, { 1, 2, 3, 4, 5 })
t:equals("c02_range_with_a_step", function() return collect(M.range(0, 10, 3)) end, { 0, 3, 6, 9 })
t:equals("c03_range_counts_down_with_a_negative_step", function() return collect(M.range(5, 1, -2)) end, { 5, 3, 1 })
t:equals("c04_an_empty_range_yields_nothing", function() return collect(M.range(3, 1)) end, {})
t:returns_error("c05_range_with_a_zero_step_returns_nil_and_an_error", function() return M.range(1, 3, 0) end)

t:scenario("c06_naturals_never_runs_out", function()
  local a, b = M.naturals(), M.naturals()
  H.expect({ a(), a(), a() }, { 1, 2, 3 }, "first three calls")
  for _ = 4, 9999 do a() end
  H.expect(a(), 10000, "call 10000")
  H.expect(b(), 1, "first call on a second generator")
end)

t:equals("c07_take_collects_the_first_n_values", function() return M.take(M.naturals(), 4) end, { 1, 2, 3, 4 })

t:scenario("c08_take_is_lazy", function()
  local calls = 0
  local function counting()
    calls = calls + 1
    return calls * 10
  end
  H.expect(M.take(counting, 3), { 10, 20, 30 }, "take(counting, 3)")
  H.expect(calls, 3, "calls to the generator after taking 3")
  H.expect(M.take(counting, 0), {}, "take(counting, 0)")
  H.expect(calls, 3, "calls to the generator after taking 0")
end)

t:equals("c09_take_stops_early_when_the_generator_ends", function() return M.take(M.range(1, 2), 5) end, { 1, 2 })

t:scenario("c10_running_total_adds_the_values_sent_with_resume", function()
  local co = M.running_total()
  H.expect(type(co), "thread", "type(running_total())")
  H.expect(coroutine.status(co), "suspended", "status before the first resume")
  H.expect({ coroutine.resume(co, 5) }, { true, 5 }, "resume(co, 5)")
  H.expect({ coroutine.resume(co, 3) }, { true, 8 }, "resume(co, 3)")
  H.expect({ coroutine.resume(co, -10) }, { true, -2 }, "resume(co, -10)")
  H.expect(coroutine.status(co), "suspended", "status between values")
  H.expect({ coroutine.resume(co) }, { true, -2 }, "resume(co) with no value")
  H.expect(coroutine.status(co), "dead", "status after it ended")
end)

t:finish()
