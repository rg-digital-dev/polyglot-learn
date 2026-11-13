-- The things in the world, as classes built from metatables:
-- Entity, and Item, Container and Npc that inherit from it.
--
-- Each class is a table of methods. Set up each one so that:
--   - its instances find methods through it (__index), and it finds the
--     methods of its parent class the same way;
--   - it can be called like its constructor: Item{...} is Item.new{...};
--   - tostring(instance) is the article and the name: "a brass lamp".

local M = {}

--- Anything in the world.
--- @class Entity
--- @field id string
--- @field name string
--- @field description string|nil
--- @field aliases string[]
--- @field article string  "a" or "an"
M.Entity = {}
M.Entity.portable = false

--- Create an entity from a table of fields; it keeps every field it is given.
--- `aliases` defaults to {}, `article` to "an" before a vowel and "a" otherwise.
--- Raises an error that blames the caller (level 2) without an id or a name.
function M.Entity.new(fields)
  error("TODO: implement Entity.new")
end

--- Whether the player could mean this entity by `word`: its id, its name, the
--- last word of its name ("lamp" for "brass lamp") or an alias, in any case.
function M.Entity:matches(word)
  error("TODO: implement Entity:matches")
end

--- What examining it shows: its description, or
--- "You see nothing special about the <name>."
function M.Entity:describe()
  error("TODO: implement Entity:describe")
end

--- Something the player can carry. Inherits from Entity.
M.Item = {}
M.Item.portable = true
M.Item.weight = 1

--- Like Entity.new, as an Item.
function M.Item.new(fields)
  error("TODO: implement Item.new")
end

--- An item that holds other items, up to `capacity` in total weight.
--- Inherits from Item.
M.Container = {}
M.Container.capacity = 10

--- Like Item.new, as a Container with empty `contents`; `open` defaults to true.
function M.Container.new(fields)
  error("TODO: implement Container.new")
end

--- The total weight of what's inside.
function M.Container:weight_inside()
  error("TODO: implement Container:weight_inside")
end

--- Put an item inside.
--- @return boolean|nil ok
--- @return string|nil err  "The box is closed.", "That won't fit in the box.",
---   "You can't put the box inside itself."
function M.Container:put(item)
  error("TODO: implement Container:put")
end

--- Take the item with this id out. @return table|nil item
function M.Container:remove(item_id)
  error("TODO: implement Container:remove")
end

--- Entity's description, then " It's closed.", " It's empty." or
--- " It contains a key and a coin."
function M.Container:describe()
  error("TODO: implement Container:describe")
end

--- Someone to talk to. Not portable. Inherits from Entity.
M.Npc = {}

--- Like Entity.new, as an Npc.
function M.Npc.new(fields)
  error("TODO: implement Npc.new")
end

--- Whether `object` is an instance of `class`, or of a class that inherits from it.
function M.is_a(object, class)
  error("TODO: implement entities.is_a")
end

--- Entities as a phrase: "a key", "a key and a coin", "a key, a coin and an apple".
function M.list(things)
  error("TODO: implement entities.list")
end

return M
