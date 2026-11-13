-- Stage 1 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in world.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local world = require("world")

local t = H.new()

-- A hall with a kitchen to the north and a garden to the east.
local function house()
  local w = world.new()
  world.add_room(w, "hall", "Hall", "A draughty hall.")
  world.add_room(w, "kitchen", "Kitchen", "It smells of bread.")
  world.add_room(w, "garden", "Garden", "Roses, mostly.")
  world.connect(w, "hall", "north", "kitchen")
  world.connect(w, "hall", "east", "garden")
  return w
end

t:scenario("c01_add_room_makes_a_room_and_the_first_is_the_start", function()
  local w = world.new()
  H.expect(w.start, nil, "a new world's start")
  local hall = world.add_room(w, "hall", "Hall", "A draughty hall.")
  H.expect(hall, { id = "hall", name = "Hall", description = "A draughty hall.", exits = {}, items = {} }, "the room")
  H.check(w.rooms.hall == hall, "the room should be stored in world.rooms by its id")
  world.add_room(w, "kitchen", "Kitchen", "It smells of bread.")
  H.expect(w.start, "hall", "the start after two rooms")
end)

t:returns_error("c02_a_room_id_can_only_be_used_once", function()
  local w = world.new()
  world.add_room(w, "hall", "Hall", "A draughty hall.")
  return world.add_room(w, "hall", "Other hall", "Again.")
end, { "hall" })

t:scenario("c03_connect_adds_the_way_back_unless_one_way", function()
  local w = house()
  H.expect(w.rooms.hall.exits, { north = "kitchen", east = "garden" }, "the hall's exits")
  H.expect(w.rooms.kitchen.exits, { south = "hall" }, "the kitchen's exits")
  world.add_room(w, "cellar", "Cellar", "Dark.")
  H.expect(world.connect(w, "kitchen", "down", "cellar", true), true, "a one-way connect")
  H.expect(w.rooms.cellar.exits, {}, "the cellar's exits after a one-way connect")
  local ok, err = world.connect(w, "hall", "sideways", "cellar")
  H.check(ok == nil and type(err) == "string", "connect with an unknown direction should return nil, err")
  ok, err = world.connect(w, "hall", "west", "attic")
  H.check(ok == nil and type(err) == "string", "connect to an unknown room should return nil, err")
end)

t:scenario("c04_exits_are_listed_in_a_fixed_order", function()
  local w = house()
  world.add_room(w, "attic", "Attic", "Dusty.")
  world.connect(w, "hall", "up", "attic")
  world.add_room(w, "porch", "Porch", "Creaky.")
  world.connect(w, "hall", "south", "porch")
  H.expect(world.exits(w, "hall"), { "north", "south", "east", "up" }, "the hall's exits")
  H.expect(world.exits(w, "garden"), { "west" }, "the garden's exits")
end)

t:scenario("c05_go_follows_an_exit_or_says_you_cant", function()
  local w = house()
  H.expect(world.go(w, "hall", "north"), "kitchen", "going north from the hall")
  H.expect({ world.go(w, "kitchen", "north") }, { nil, "You can't go that way." }, "going north from the kitchen")
end)

t:scenario("c06_place_and_remove_move_items_in_and_out_of_a_room", function()
  local w = house()
  local lamp, rope, key = { id = "lamp", name = "brass lamp" }, { id = "rope", name = "rope" }, { id = "key", name = "key" }
  H.check(world.place(w, "hall", lamp) == lamp, "place should return the item")
  world.place(w, "hall", rope)
  world.place(w, "hall", key)
  H.check(world.remove(w, "hall", "rope") == rope, "remove should return the item it took out")
  H.expect(w.rooms.hall.items, { lamp, key }, "the hall's items after removing the rope")
  H.expect(world.remove(w, "hall", "rope"), nil, "removing an item that isn't there")
end)

t:scenario("c07_describe_shows_the_name_text_exits_and_items", function()
  local w = house()
  world.place(w, "hall", { id = "lamp", name = "brass lamp" })
  world.place(w, "hall", { id = "rope", name = "rope" })
  H.expect(world.describe(w, "hall"), "Hall\nA draughty hall.\nExits: north, east.\nYou see: brass lamp, rope.", "the hall")
  H.expect(world.describe(w, "kitchen"), "Kitchen\nIt smells of bread.\nExits: south.", "the kitchen, with nothing in it")
  world.add_room(w, "void", "Void", "Nothing at all.")
  H.expect(world.describe(w, "void"), "Void\nNothing at all.\nExits: none.", "a room without exits")
end)

t:scenario("c08_build_makes_a_world_from_a_definition_table", function()
  local w = world.build({
    start = "hall",
    rooms = {
      kitchen = { name = "Kitchen", description = "It smells of bread.", exits = { south = "hall" } },
      hall = { name = "Hall", description = "A draughty hall.", exits = { north = "kitchen", up = "attic" } },
      attic = { name = "Attic", description = "Dusty." },
    },
  })
  H.check(type(w) == "table", "build should return the world")
  H.expect(w.start, "hall", "the start")
  H.expect(world.exits(w, "hall"), { "north", "up" }, "the hall's exits")
  H.expect(w.rooms.attic.exits, {}, "exits are one-way as written: the attic's exits")
  local bad, err = world.build({ rooms = { hall = { name = "Hall", description = "", exits = { north = "nowhere" } } } })
  H.check(bad == nil and type(err) == "string" and err:find("nowhere", 1, true), "an exit to an unknown room should return nil and an error naming it")
  bad, err = world.build({ start = "moon", rooms = { hall = { name = "Hall", description = "" } } })
  H.check(bad == nil and type(err) == "string" and err:find("moon", 1, true), "an unknown start should return nil and an error naming it")
end)

t:finish()
