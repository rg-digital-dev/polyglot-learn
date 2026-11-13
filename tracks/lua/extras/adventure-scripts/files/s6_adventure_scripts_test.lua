-- Stage 6 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in scripts.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local scripts = require("scripts")

local t = H.new()

-- The ferryman wants a coin; the outcome is what the script returns.
local function ferryman(say, ask)
  say("The ferryman looks up.")
  say("\"Crossing?\"")
  local answer = ask("\"It'll cost you a coin.\"", { "pay", "refuse" })
  if answer == "pay" then
    say("\"Hop in.\"")
    return "crossed"
  end
  local sure = ask("\"Sure?\"", { "Yes", "No" })
  say("\"Suit yourself.\"")
  return "stayed:" .. sure
end

t:scenario("c37_a_conversation_runs_to_its_first_question", function()
  local started = false
  local conv = scripts.conversation(function(say, ask) started = true; return ferryman(say, ask) end)
  H.expect({ started, conv.done }, { false, false }, "started and done before the first next()")
  H.expect(conv:next(), "The ferryman looks up.\n\"Crossing?\"\n\"It'll cost you a coin.\"\n1) pay  2) refuse", "the first next()")
  H.expect(conv.done, false, "done while it waits for an answer")
end)

t:scenario("c38_next_resumes_with_the_answer_and_ask_returns_it", function()
  local conv = scripts.conversation(ferryman)
  conv:next()
  H.expect(conv:next("refuse"), "\"Sure?\"\n1) Yes  2) No", "answering refuse")
  H.expect(conv:next("1"), "\"Suit yourself.\"", "answering 1")
  H.expect({ conv.done, conv.result }, { true, "stayed:Yes" }, "done and result")
end)

t:scenario("c39_answers_must_be_one_of_the_choices", function()
  local conv = scripts.conversation(ferryman)
  conv:next()
  H.expect({ conv:next("swim") }, { nil, "Please answer with one of: pay, refuse." }, "answering swim")
  H.expect({ conv:next("3") }, { nil, "Please answer with one of: pay, refuse." }, "answering 3")
  H.expect(conv:next("  PAY "), "\"Hop in.\"", "answering PAY")
  H.expect({ conv.done, conv.result }, { true, "crossed" }, "done and result")
end)

t:scenario("c40_a_finished_conversation_has_a_result_and_says_it_is_over", function()
  local conv = scripts.conversation(function(say) say("\"Lovely weather.\"") return 42 end)
  H.expect(conv:next(), "\"Lovely weather.\"", "a conversation without questions")
  H.expect({ conv.done, conv.result }, { true, 42 }, "done and result")
  H.expect({ conv:next("hello?") }, { nil, "The conversation is over." }, "next() after the end")
  local open = scripts.conversation(function(say, ask) return ask("\"Name?\"") end)
  H.expect(open:next(), "\"Name?\"", "a question without choices")
  H.expect(open:next("Ann"), "", "any answer to a question without choices")
  H.expect(open.result, "Ann", "the result")
end)

t:scenario("c41_an_error_in_a_script_ends_the_conversation", function()
  local conv = scripts.conversation(function(say, ask)
    ask("\"Ready?\"", { "yes" })
    error("the ferry sank", 0)
  end)
  conv:next()
  H.expect({ conv:next("yes") }, { nil, "the ferry sank" }, "the answer that hits the error")
  H.expect(conv.done, true, "done after the error")
end)

t:scenario("c42_the_scheduler_runs_scripts_a_step_per_tick", function()
  local s = scripts.scheduler()
  local log = {}
  s:spawn("bell", function(times)
    for i = 1, times do
      log[#log + 1] = "bell " .. i
      scripts.wait(2)
    end
  end, 2)
  s:spawn("crow", function()
    log[#log + 1] = "crow"
    scripts.wait()
    log[#log + 1] = "crow again"
  end)
  H.expect(log, {}, "nothing runs before the first tick")
  H.expect(s:running(), { "bell", "crow" }, "running() after spawning")
  local seen = {}
  for tick = 1, 5 do
    H.expect(s:tick(), {}, "errors at tick " .. tick)
    seen[tick] = table.concat(log, ", ")
    log = {}
  end
  H.expect(seen, { "bell 1, crow", "crow again", "bell 2", "", "" }, "what ran at ticks 1 to 5")
  H.expect(s:running(), {}, "running() once both finished")
  local ok, err = pcall(scripts.wait, 1)
  H.check(not ok and tostring(err):find("outside a script", 1, true), "wait() outside a script should raise, got: " .. tostring(err))
end)

t:scenario("c43_a_failing_script_is_dropped_and_reported", function()
  local s = scripts.scheduler()
  local count = 0
  s:spawn("ok", function() while true do count = count + 1; scripts.wait() end end)
  s:spawn("bad", function() scripts.wait(); error("out of ink", 0) end)
  H.expect(s:tick(), {}, "errors at tick 1")
  H.expect(s:tick(), { "bad: out of ink" }, "errors at tick 2")
  H.expect(s:tick(), {}, "errors at tick 3")
  H.expect({ count, s:running() }, { 3, { "ok" } }, "the other script kept running")
end)

t:finish()
