-- The map: rooms, the exits between them, and what lies in each room.
-- A world is a plain table: { rooms = { [id] = room }, start = id }, and a
-- room is { id, name, description, exits = { [direction] = room id }, items = { ... } }.

local M = {}

--- Every direction, in the order exits are listed.
M.DIRECTIONS = { "north", "south", "east", "west", "up", "down" }

--- The way back for each direction.
M.OPPOSITE = { north = "south", south = "north", east = "west", west = "east", up = "down", down = "up" }

--- An empty world: no rooms, no start yet.
function M.new()
  error("TODO: implement world.new")
end

--- Add a room. The first room added becomes the world's start.
--- @return table|nil room  { id, name, description, exits = {}, items = {} }
--- @return string|nil err  if the id is already taken
function M.add_room(world, id, name, description)
  error("TODO: implement world.add_room")
end

--- Connect `from` to `to` going `direction`, and `to` back to `from` the
--- opposite way unless `one_way` is true.
--- @return boolean|nil ok
--- @return string|nil err  for an unknown room or direction
function M.connect(world, from, direction, to, one_way)
  error("TODO: implement world.connect")
end

--- The directions out of a room, in DIRECTIONS order.
--- @return string[]
function M.exits(world, room_id)
  error("TODO: implement world.exits")
end

--- The room id that `direction` leads to from `room_id`.
--- @return string|nil to
--- @return string|nil err  "You can't go that way."
function M.go(world, room_id, direction)
  error("TODO: implement world.go")
end

--- Put an item (any table with an id and a name) at the end of a room's items.
--- @return table item
function M.place(world, room_id, item)
  error("TODO: implement world.place")
end

--- Take the item with this id out of a room, keeping the others in order.
--- @return table|nil item  nil if it isn't there
function M.remove(world, room_id, item_id)
  error("TODO: implement world.remove")
end

--- The room as the player sees it:
---   Hall
---   A draughty hall.
---   Exits: north, east.
---   You see: brass lamp, rope.
--- "Exits: none." when there are none; no "You see" line for an empty room.
function M.describe(world, room_id)
  error("TODO: implement world.describe")
end

--- Build a world from a definition table:
---   { start = "hall", rooms = { hall = { name = ..., description = ..., exits = { north = "kitchen" } }, ... } }
--- Exits are one-way, as written. Rooms are added in id order, so without a
--- `start` the first id in sorted order is the start.
--- @return table|nil world
--- @return string|nil err  for an exit to a room that doesn't exist, or an unknown start
function M.build(def)
  error("TODO: implement world.build")
end

return M
