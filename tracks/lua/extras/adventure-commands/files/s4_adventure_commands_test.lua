-- Stage 4 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in commands.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local world = require("world")
local entities = require("entities")
local commands = require("commands")

local t = H.new()

-- A hall with a lamp, a statue and an open chest holding a key; a kitchen to the north.
local function game()
  local w = world.new()
  world.add_room(w, "hall", "Hall", "A draughty hall.")
  world.add_room(w, "kitchen", "Kitchen", "It smells of bread.")
  world.connect(w, "hall", "north", "kitchen")
  world.place(w, "hall", entities.Item({ id = "lamp", name = "brass lamp", description = "It glows." }))
  world.place(w, "hall", entities.Entity({ id = "statue", name = "statue" }))
  local chest = entities.Container({ id = "chest", name = "chest", portable = false, description = "Oak." })
  chest:put(entities.Item({ id = "key", name = "iron key" }))
  world.place(w, "hall", chest)
  return commands.new_state(w), w
end

local function run(state, verb, object, prep, target)
  return commands.run(state, { verb = verb, object = object, prep = prep, target = target })
end

local function ids(list)
  local out = {}
  for i, e in ipairs(list) do out[i] = e.id end
  return out
end

t:scenario("c22_new_state_starts_in_the_start_room_with_empty_hands", function()
  local state, w = game()
  H.check(state.world == w, "state.world should be the world")
  H.expect({ state.room, state.inventory, state.flags, state.turns }, { "hall", {}, {}, 0 }, "room, inventory, flags and turns")
  H.check(state.handlers.look == commands.handlers.look, "state.handlers should fall through to commands.handlers")
  H.check(rawget(state.handlers, "look") == nil, "the defaults shouldn't be copied into state.handlers")
end)

t:scenario("c23_look_and_go_describe_rooms_and_move_the_player", function()
  local state = game()
  H.expect(run(state, "look"), "Hall\nA draughty hall.\nExits: north.\nYou see: brass lamp, statue, chest.", "look")
  H.expect(run(state, "go", "north"), "Kitchen\nIt smells of bread.\nExits: south.", "go north")
  H.expect(state.room, "kitchen", "the room after going north")
  H.expect({ run(state, "go", "east") }, { nil, "You can't go that way." }, "go east from the kitchen")
  H.expect({ run(state, "go") }, { nil, "Go where?" }, "go without a direction")
  H.expect(state.room, "kitchen", "the room after failing to move")
  H.expect(state.turns, 2, "turns: only commands that worked count")
end)

t:scenario("c24_take_moves_a_portable_item_into_the_inventory", function()
  local state, w = game()
  H.expect(run(state, "take", "lamp"), "Taken.", "take lamp")
  H.expect(ids(state.inventory), { "lamp" }, "the inventory")
  H.expect(ids(w.rooms.hall.items), { "statue", "chest" }, "the hall's items")
  H.expect({ run(state, "take", "lamp") }, { nil, "You already have the brass lamp." }, "take lamp again")
  H.expect({ run(state, "take", "statue") }, { nil, "You can't take the statue." }, "take statue")
  H.expect({ run(state, "take", "piano") }, { nil, "There is no piano here." }, "take piano")
  H.expect({ run(state, "take") }, { nil, "Take what?" }, "take with no object")
end)

t:scenario("c25_drop_puts_a_carried_item_in_the_room", function()
  local state, w = game()
  run(state, "take", "lamp")
  run(state, "go", "north")
  H.expect(run(state, "drop", "brass lamp"), "Dropped.", "drop brass lamp")
  H.expect({ ids(state.inventory), ids(w.rooms.kitchen.items) }, { {}, { "lamp" } }, "inventory and kitchen")
  H.expect({ run(state, "drop", "lamp") }, { nil, "You don't have a lamp." }, "drop lamp again")
end)

t:scenario("c26_inventory_lists_what_you_carry", function()
  local state = game()
  H.expect(run(state, "inventory"), "You are empty-handed.", "inventory with nothing")
  run(state, "take", "lamp")
  H.expect(run(state, "inventory"), "You are carrying a brass lamp.", "inventory with the lamp")
  run(state, "take", "key")
  H.expect(run(state, "inventory"), "You are carrying a brass lamp and an iron key.", "inventory with the lamp and the key")
end)

t:scenario("c27_find_and_take_reach_inside_open_containers", function()
  local state, w = game()
  local chest = w.rooms.hall.items[3]
  local key, holder = commands.find(state, "key")
  H.check(key ~= nil and key.id == "key" and holder == chest, "find(state, \"key\") should return the key and the chest")
  H.expect(run(state, "take", "key"), "Taken.", "take key")
  H.expect({ ids(state.inventory), chest.contents }, { { "key" }, {} }, "inventory and chest")
  run(state, "take", "lamp")
  H.expect(run(state, "put", "lamp", "in", "chest"), "You put the brass lamp in the chest.", "put lamp in chest")
  H.expect(ids(chest.contents), { "lamp" }, "the chest's contents")
  H.expect({ run(state, "put", "key", "in", "statue") }, { nil, "You can't put things in the statue." }, "put key in statue")
  chest.open = false
  H.expect({ run(state, "put", "key", "in", "chest") }, { nil, "The chest is closed." }, "put key in the closed chest")
  H.expect({ commands.find(state, "lamp") }, { nil, nil, "There is no lamp here." }, "find in a closed chest")
end)

t:scenario("c28_examine_describes_things_here_or_carried", function()
  local state = game()
  H.expect(run(state, "examine", "lamp"), "It glows.", "examine lamp")
  H.expect(run(state, "examine", "chest"), "Oak. It contains an iron key.", "examine chest")
  run(state, "take", "lamp")
  run(state, "go", "north")
  H.expect(run(state, "examine", "lamp"), "It glows.", "examine the lamp you carry")
end)

t:scenario("c29_unknown_verbs_fail_and_a_state_can_add_its_own", function()
  local state = game()
  H.expect({ run(state, "dance") }, { nil, "You can't dance here." }, "run dance")
  H.expect(commands.register(state, "dance", function() return "You dance." end), nil, "register returns the handler it replaces")
  H.expect(run(state, "dance"), "You dance.", "dance after register")
  local previous
  previous = commands.register(state, "look", function(s, c) return "[" .. previous(s, c) .. "]" end)
  H.check(previous == commands.handlers.look, "register(\"look\") should return the default look")
  H.expect(run(state, "look"):sub(1, 6), "[Hall\n", "a look that wraps the default")
  local other = game()
  H.expect({ run(other, "dance") }, { nil, "You can't dance here." }, "another state doesn't get the new verb")
  H.check(commands.handlers.dance == nil, "register shouldn't change commands.handlers")
end)

t:finish()
