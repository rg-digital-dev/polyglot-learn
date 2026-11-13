-- Events: listeners that run when something happens, and can stop it.

local M = {}

--- The emitter class: instances are { listeners = { [event] = { ... } } }.
local Emitter = {}
Emitter.__index = Emitter
M.Emitter = Emitter

--- A new emitter with no listeners.
function M.new()
  error("TODO: implement events.new")
end

--- Call fn(...) on every emit of `event`, after the listeners added before it.
--- @return function off  removes this listener; calling it again does nothing
function Emitter:on(event, fn)
  error("TODO: implement Emitter:on")
end

--- Like on, but the listener is removed before it runs the first time.
--- @return function off
function Emitter:once(event, fn)
  error("TODO: implement Emitter:once")
end

--- How many listeners `event` has.
function Emitter:count(event)
  error("TODO: implement Emitter:count")
end

--- Call the listeners of `event` in order with `...`. A listener that returns
--- false (and a reason) vetoes the event: the rest don't run.
--- Listeners may add or remove listeners while it runs; that takes effect on
--- the next emit.
--- @return boolean ok  false when vetoed
--- @return integer|string|nil  how many listeners ran, or the veto's reason
function Emitter:emit(event, ...)
  error("TODO: implement Emitter:emit")
end

--- Call object[method](object, ...) on every emit of `event`, without keeping
--- `object` alive: once it is garbage collected, the listener removes itself.
--- @return function off
function Emitter:watch(event, object, method)
  error("TODO: implement Emitter:watch")
end

--- A command handler that emits "before <verb>" with (state, command) first,
--- returning nil and the reason (or "You can't do that.") if it is vetoed;
--- then runs `handler`, and emits "after <verb>" with (state, command, text)
--- when it succeeded.
function M.wrap(emitter, verb, handler)
  error("TODO: implement events.wrap")
end

return M
