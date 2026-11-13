-- What each verb does: a table of handler functions, looked up by verb.

local world = require("world")
local entities = require("entities")

local M = {}

--- The default handlers: verb -> function(state, command) that returns the
--- text to show, or nil, err when the command can't be done. A command is
--- what parser.parse returns: { verb, object, prep, target }.
M.handlers = {}

--- A new game state in the world's start room:
---   { world = w, room = w.start, inventory = {}, flags = {}, turns = 0, handlers = ... }
--- `handlers` is the state's own table of verbs; it falls through to
--- M.handlers through its metatable, so a game can add or replace verbs
--- without changing anyone else's.
function M.new_state(w)
  error("TODO: implement commands.new_state")
end

--- Set this state's handler for a verb; returns the handler it replaces
--- (its own or the default), so the new one can call it.
function M.register(state, verb, handler)
  error("TODO: implement commands.register")
end

--- The entity `word` refers to (see Entity:matches): in the room, then in the
--- inventory, then inside an open container in either.
--- @return table|nil entity
--- @return table|nil holder  the list or the container it's in
--- @return string|nil err  "There is no lamp here."
function M.find(state, word)
  error("TODO: implement commands.find")
end

--- The room's description.
function M.handlers.look(state)
  error("TODO: implement the look handler")
end

--- Move through an exit and describe the new room.
--- Errors: "Go where?", or world.go's error.
function M.handlers.go(state, command)
  error("TODO: implement the go handler")
end

--- "Taken." Moves a portable entity from the room or a container to the inventory.
--- Errors: "Take what?", find's error, "You already have the brass lamp.",
--- "You can't take the statue."
function M.handlers.take(state, command)
  error("TODO: implement the take handler")
end

--- "Dropped." Moves a carried entity to the room.
--- Errors: "Drop what?", "You don't have a lamp." (the word as typed)
function M.handlers.drop(state, command)
  error("TODO: implement the drop handler")
end

--- "You are carrying a brass lamp and an iron key." or "You are empty-handed."
function M.handlers.inventory(state)
  error("TODO: implement the inventory handler")
end

--- The entity's describe(). Errors: "Examine what?", find's error.
function M.handlers.examine(state, command)
  error("TODO: implement the examine handler")
end

--- "You put the brass lamp in the chest." Moves a carried entity into a container.
--- Errors: "Put what where?", "You don't have a lamp.", find's error for the
--- target, "You can't put things in the statue." (not a Container), or put's error.
function M.handlers.put(state, command)
  error("TODO: implement the put handler")
end

--- Run a parsed command with the state's handler for its verb. A command
--- that succeeds (returns text) counts as a turn.
--- @return string|nil text
--- @return string|nil err  "You can't xyzzy here." when there is no handler
function M.run(state, command)
  error("TODO: implement commands.run")
end

return M
