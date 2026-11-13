-- Scripts that pause: conversations that wait for the player's answers, and
-- background scripts that run a step per turn. Both are coroutines.

local M = {}

local Conversation = {}
Conversation.__index = Conversation

--- A conversation running script(say, ask) as a coroutine. say(text) adds a
--- line; ask(question, choices) shows the lines so far and the question, waits
--- for the player's answer and returns the chosen choice (or, without
--- choices, the answer as typed). What the script returns becomes `result`
--- when it ends, and `done` becomes true. Nothing runs until the first next().
function M.conversation(script)
  error("TODO: implement scripts.conversation")
end

--- Run the script up to its next question, or to its end.
--- The first call starts it (`answer` is ignored); later calls pass the answer.
--- A choice can be picked by its number ("2") or its text, in any case and
--- with spaces around it.
--- @return string|nil text  the lines said, one per line, then the question
---   and its numbered choices on one line: "1) yes  2) no"
--- @return string|nil err  'Please answer with one of: yes, no.' for an answer
---   that isn't a choice (the script stays where it was), "The conversation is
---   over." after the end, or the script's error (which ends it)
function Conversation:next(answer)
  error("TODO: implement Conversation:next")
end

--- Inside a scheduled script: pause for `n` ticks (default 1).
--- Raises "wait() outside a script" when not called from one.
function M.wait(n)
  error("TODO: implement scripts.wait")
end

local Scheduler = {}
Scheduler.__index = Scheduler

--- A scheduler with no scripts, at tick 0.
function M.scheduler()
  error("TODO: implement scripts.scheduler")
end

--- Add a script: fn(...) runs as a coroutine from the next tick on.
function Scheduler:spawn(name, fn, ...)
  error("TODO: implement Scheduler:spawn")
end

--- Advance one tick: resume, in the order they were spawned, every script
--- whose wait is over, up to its next wait(). Finished scripts are dropped,
--- and so are scripts that raise an error.
--- @return string[] errors  "name: message" for each script that failed
function Scheduler:tick()
  error("TODO: implement Scheduler:tick")
end

--- The names of the scripts still running, in spawn order.
function Scheduler:running()
  error("TODO: implement Scheduler:running")
end

return M
