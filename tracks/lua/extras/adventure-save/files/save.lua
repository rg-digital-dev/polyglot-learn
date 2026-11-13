-- Saving and loading: the game's state as a Lua table literal in a file, read
-- back with load() in an empty environment.

local M = {}

--- Lua source that rebuilds `value`: "return " followed by a literal.
---   serialize({ room = "hall", inventory = { "lamp" } })
---   --> 'return {inventory = {"lamp"}, room = "hall"}'
--- Tables list their sequence first, then the other keys sorted (numbers,
--- then strings), separated by ", "; names that are identifiers (and not
--- keywords) are written bare, others in brackets: [5], ["two words"].
--- Strings use %q. Integers and floats stay what they are (1 and 1.0), and
--- floats come back exactly; math.huge is written 1/0.
--- Raises "can't save a function at state.flags.f" for a function (or any
--- other value without a literal) and "can't save a cycle at <path>" for a
--- table that contains itself; the top level's path is "state", then ".key"
--- or "[1]" for each step down.
function M.serialize(value)
  error("TODO: implement save.serialize")
end

--- At most this many Lua instructions per deserialize; a save is only data.
M.STEP_LIMIT = 1000000

--- Read what serialize wrote. The text runs as a text chunk only (no
--- bytecode), with an empty table as its environment, so it can't reach
--- os, io or any other global, and it is stopped after STEP_LIMIT
--- instructions.
--- @return any|nil value
--- @return string|nil err  for a syntax error, a runtime error or a timeout
function M.deserialize(text)
  error("TODO: implement save.deserialize")
end

--- Every entity in the game by id: in the rooms, the inventory, and inside
--- containers at any depth.
--- @return table<string, table>
function M.index(state)
  error("TODO: implement save.index")
end

--- The state as plain data, by ids:
---   { room = "hall", turns = 3, flags = {...}, inventory = { "lamp" },
---     rooms = { hall = { "rope" }, ... }, containers = { box = { "key" } } }
--- `rooms` has every room, also empty ones; `containers` every container.
function M.snapshot(state)
  error("TODO: implement save.snapshot")
end

--- Put the state back the way a snapshot describes it: the room, turns,
--- flags, inventory, each room's items and each container's contents, using
--- the game's own entities (found with index).
--- @return boolean|nil ok
--- @return string|nil err  "unknown item 'x' in the save" or "unknown room 'x'
---   in the save"; on an error the state is left unchanged
function M.restore(state, snap)
  error("TODO: implement save.restore")
end

--- Write the state's snapshot to a file.
--- @return boolean|nil ok
--- @return string|nil err  io.open's error
function M.save(state, path)
  error("TODO: implement save.save")
end

--- Read a file written by save and restore the state from it.
--- @return boolean|nil ok
--- @return string|nil err  io.open's, deserialize's or restore's error
function M.load(state, path)
  error("TODO: implement save.load")
end

return M
