-- Stage 5 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in events.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local events = require("events")

local t = H.new()

t:scenario("c30_emit_calls_the_listeners_in_order_with_the_arguments", function()
  local e = events.new()
  local log = {}
  e:on("take", function(who, what) log[#log + 1] = "first " .. who .. " " .. what end)
  e:on("take", function(who, what, extra) log[#log + 1] = "second " .. who .. " " .. what .. " " .. tostring(extra) end)
  e:on("drop", function() log[#log + 1] = "drop" end)
  H.expect({ e:emit("take", "ann", "lamp") }, { true, 2 }, "emit(\"take\", ...)")
  H.expect(log, { "first ann lamp", "second ann lamp nil" }, "what the listeners saw")
  H.expect({ e:emit("nothing") }, { true, 0 }, "emit with no listeners")
  H.expect({ e:count("take"), e:count("drop"), e:count("nothing") }, { 2, 1, 0 }, "count")
end)

t:scenario("c31_on_returns_a_function_that_unsubscribes", function()
  local e = events.new()
  local calls = 0
  local off = e:on("tick", function() calls = calls + 1 end)
  e:emit("tick")
  off()
  off() -- a second call does nothing
  e:emit("tick")
  H.expect({ calls, e:count("tick") }, { 1, 0 }, "calls and listeners after off()")
end)

t:scenario("c32_listeners_can_unsubscribe_while_emit_runs", function()
  local e = events.new()
  local log = {}
  local off_a
  off_a = e:on("tick", function() log[#log + 1] = "a"; off_a() end)
  e:on("tick", function() log[#log + 1] = "b" end)
  e:on("tick", function() log[#log + 1] = "c"; e:on("tick", function() log[#log + 1] = "late" end) end)
  H.expect({ e:emit("tick") }, { true, 3 }, "the first emit")
  H.expect(log, { "a", "b", "c" }, "the first emit runs a, b and c, not the listener added during it")
  log = {}
  e:emit("tick")
  H.expect(log, { "b", "c", "late" }, "the second emit")
end)

t:scenario("c33_once_runs_a_listener_a_single_time", function()
  local e = events.new()
  local seen = {}
  e:once("open", function(what) seen[#seen + 1] = what end)
  e:emit("open", "door")
  e:emit("open", "box")
  H.expect({ seen, e:count("open") }, { { "door" }, 0 }, "what once saw, and the listeners left")
  local off = e:once("open", function() seen[#seen + 1] = "never" end)
  off()
  e:emit("open", "lid")
  H.expect(seen, { "door" }, "a once listener removed before it ran")
end)

t:scenario("c34_a_listener_vetoes_by_returning_false_and_a_reason", function()
  local e = events.new()
  local ran = false
  e:on("go", function() return nil end) -- nil is not a veto
  e:on("go", function(direction) if direction == "down" then return false, "It's too dark." end end)
  e:on("go", function() ran = true end)
  H.expect({ e:emit("go", "down") }, { false, "It's too dark." }, "emit(\"go\", \"down\")")
  H.expect(ran, false, "the listener after the veto ran")
  H.expect({ e:emit("go", "up") }, { true, 3 }, "emit(\"go\", \"up\")")
end)

t:scenario("c35_watch_lets_its_object_be_garbage_collected", function()
  local e = events.new()
  local Lamp = {}
  Lamp.__index = Lamp
  function Lamp:hear(sound) self.heard[#self.heard + 1] = sound end
  local kept = setmetatable({ heard = {} }, Lamp)
  e:watch("bang", kept, "hear")
  local function watch_a_lamp_nobody_keeps()
    e:watch("bang", setmetatable({ heard = {} }, Lamp), "hear")
  end
  watch_a_lamp_nobody_keeps()
  collectgarbage("collect")
  collectgarbage("collect")
  e:emit("bang", "boom")
  H.expect(kept.heard, { "boom" }, "what the kept lamp heard")
  H.expect(e:count("bang"), 1, "listeners left once the other lamp was collected")
  local off = e:watch("bang", kept, "hear")
  off()
  e:emit("bang", "pop")
  H.expect(kept.heard, { "boom", "pop" }, "after a second watch was turned off")
end)

t:scenario("c36_wrap_emits_before_and_after_around_a_handler", function()
  local e = events.new()
  local log = {}
  local handler = events.wrap(e, "take", function(state, command)
    log[#log + 1] = "handler " .. command.object
    if command.object == "ghost" then return nil, "Your hand goes right through it." end
    return "Taken."
  end)
  e:on("before take", function(state, command)
    log[#log + 1] = "before " .. command.object
    if command.object == "statue" then return false, "It's far too heavy." end
  end)
  e:on("after take", function(state, command, text) log[#log + 1] = "after " .. command.object .. ": " .. text end)
  local state = {}
  H.expect(handler(state, { verb = "take", object = "lamp" }), "Taken.", "taking the lamp")
  H.expect({ handler(state, { verb = "take", object = "statue" }) }, { nil, "It's far too heavy." }, "taking the statue")
  H.expect({ handler(state, { verb = "take", object = "ghost" }) }, { nil, "Your hand goes right through it." }, "taking the ghost")
  H.expect(log, { "before lamp", "handler lamp", "after lamp: Taken.", "before statue", "before ghost", "handler ghost" }, "the order things ran in")
  e:on("before take", function() return false end)
  H.expect({ handler(state, { verb = "take", object = "lamp" }) }, { nil, "You can't do that." }, "a veto without a reason")
end)

t:finish()
