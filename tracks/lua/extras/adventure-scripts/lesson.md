# Scripts that pause, with coroutines

A conversation in a game is a small program. "Say this, ask that, and depending on the answer, say something else." But the answer arrives later, in the next call to the game's input. Without coroutines you would turn the conversation into a state machine by hand. With a coroutine, the script is written top to bottom and simply *pauses* at each question.

`coroutine.yield(x)` stops the coroutine and hands `x` to whoever resumed it. `coroutine.resume(co, y)` continues it, and inside, `yield` returns `y`. Values travel both ways through the same pair of calls.

```lua
local quiz = coroutine.create(function()
  local name = coroutine.yield("What's your name?")
  local age = coroutine.yield("How old are you, " .. name .. "?")
  return name .. " is " .. age
end)

print(coroutine.resume(quiz))           --> true   What's your name?
print(coroutine.resume(quiz, "Ann"))    --> true   How old are you, Ann?
print(coroutine.resume(quiz, "31"))     --> true   Ann is 31
print(coroutine.status(quiz))           --> dead
print(coroutine.resume(quiz))           --> false  cannot resume dead coroutine
```

`resume` never raises. It returns `false` and the error when the coroutine fails, and that ends the coroutine. A scheduler uses the same mechanism: each background script yields how long it wants to sleep, and the scheduler resumes it when that time has come. Everything runs on one thread, one step at a time, so there are no locks and no races.

<!-- from:typescript -->
## Coming from TypeScript
- A coroutine is a generator: `yield` pauses it, and `resume(co, value)` is `gen.next(value)`. The difference is that a Lua coroutine can yield from inside any function it calls, not only from its own body. That's why `say` and `ask` can be plain functions that yield on the script's behalf.
- There is no `async`/`await`, and nothing runs by itself. A coroutine only moves when you resume it.
- `coroutine.wrap(fn)` returns a function that resumes, like iterating a generator. It raises errors instead of returning them.
<!-- /from -->

<!-- from:default -->
## Key points
- `yield` passes a value out and returns the value of the next `resume`.
- `resume` returns `true` plus the values, or `false` plus the error. A dead coroutine stays dead.
- `coroutine.isyieldable()` tells you whether you're inside a coroutine that can yield.
<!-- /from -->

## Common mistake
Calling `coroutine.resume` and checking only the value:

```lua
local _, question = coroutine.resume(co, answer)
print(question)        -- after an error inside the script, this prints the error message as if it were a question
```

Always look at the first result. When it is `false`, the second one is an error, not a question.

## In this stage
In `scripts.lua`, implement `conversation` and its `next`, then `wait`, `scheduler`, and the scheduler's `spawn`, `tick` and `running`. The conversation and scheduler classes are already declared at the top of the file.