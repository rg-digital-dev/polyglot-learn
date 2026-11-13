-- Stage 7 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in save.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local world = require("world")
local entities = require("entities")
local commands = require("commands")
local save = require("save")

local t = H.new()

-- A file name in the system's temporary folder.
local function temp_path(name)
  local base = os.getenv("TMPDIR") or os.getenv("TEMP") or os.getenv("TMP") or "/tmp"
  return base .. "/" .. name .. "-" .. tostring(os.time()) .. "-" .. tostring(math.random(1e9))
end

local function serializes(value, expected, what)
  H.expect(save.serialize(value), expected, "serialize(" .. what .. ")")
end

-- Run serialize's output with plain load, to see what it rebuilds.
local function rebuild(value)
  return assert(load(save.serialize(value), "=test", "t", {}))()
end

local function raises(fn, mentions)
  local ok, err = pcall(fn)
  H.check(not ok, "expected an error, but the call returned normally")
  H.check(not tostring(err):find("TODO: implement", 1, true), "raised the stub's error: " .. tostring(err))
  H.check(tostring(err):find(mentions, 1, true), "the error " .. string.format("%q", tostring(err)) .. " should mention " .. string.format("%q", mentions))
end

t:scenario("c44_serialize_writes_plain_values_as_lua_source", function()
  serializes(nil, "return nil", "nil")
  serializes(true, "return true", "true")
  serializes(42, "return 42", "42")
  serializes(-7, "return -7", "-7")
  serializes(1.5, "return 1.5", "1.5")
  H.expect(math.type(rebuild(1.0)), "float", "1.0 should come back a float")
  H.expect(rebuild(0.1), 0.1, "0.1 should come back exactly")
  H.expect(rebuild(math.huge), math.huge, "math.huge")
  local text = "a \"quoted\"\nline\0with a zero"
  H.expect(rebuild(text), text, "a string with quotes, a newline and a zero byte")
end)

t:scenario("c45_tables_list_their_sequence_then_sorted_keys", function()
  serializes({}, "return {}", "{}")
  serializes({ "a", "b" }, 'return {"a", "b"}', "a list")
  serializes({ room = "hall", turns = 3, flags = { lit = true } }, 'return {flags = {lit = true}, room = "hall", turns = 3}', "a record")
  serializes({ 10, 20, [5] = 50, ["two words"] = 2, ["end"] = 1, x = { 1 } }, 'return {10, 20, [5] = 50, ["end"] = 1, ["two words"] = 2, x = {1}}', "a mixed table")
end)

t:scenario("c46_serialize_refuses_functions_and_cycles_and_names_the_path", function()
  raises(function() return save.serialize({ flags = { f = print } }) end, "can't save a function at state.flags.f")
  local loop = { inventory = {} }
  loop.inventory[1] = loop
  raises(function() return save.serialize(loop) end, "can't save a cycle at state.inventory[1]")
  local shared = { 1 }
  H.expect(rebuild({ a = shared, b = shared }), { a = { 1 }, b = { 1 } }, "the same table twice is not a cycle")
end)

t:scenario("c47_deserialize_reads_what_serialize_wrote", function()
  local value = { room = "hall", turns = 12, inventory = { "lamp", "key" }, flags = { lit = true, ratio = 0.25 }, [3] = "three" }
  H.expect(save.deserialize(save.serialize(value)), value, "a round trip")
  H.expect(save.deserialize("return 'hand written'"), "hand written", "deserialize of hand-written text")
  local v, err = save.deserialize("return {")
  H.check(v == nil and type(err) == "string", "a syntax error should return nil, err")
end)

t:scenario("c48_deserialize_runs_with_no_globals_and_a_step_limit", function()
  local v, err = save.deserialize("return os.exit(1)")
  H.check(v == nil and type(err) == "string" and err:find("os", 1, true), "a save can't reach os: expected nil and an error, got " .. tostring(v) .. ", " .. tostring(err))
  v, err = save.deserialize("x = 1 return x")
  H.check(v == 1 and rawget(_G, "x") == nil, "a save's globals go into its own empty table, not _G")
  v, err = save.deserialize("while true do end")
  H.check(v == nil and type(err) == "string", "an endless loop should be stopped and return nil, err")
  H.check(debug.gethook() == nil, "deserialize should remove its hook afterwards")
  v, err = save.deserialize(string.dump(function() return 1 end))
  H.check(v == nil and type(err) == "string", "bytecode should be refused")
end)

-- A hall with a lamp and a chest holding a key; a kitchen to the north.
local function game()
  local w = world.new()
  world.add_room(w, "hall", "Hall", "A draughty hall.")
  world.add_room(w, "kitchen", "Kitchen", "It smells of bread.")
  world.connect(w, "hall", "north", "kitchen")
  world.place(w, "hall", entities.Item({ id = "lamp", name = "lamp" }))
  local chest = entities.Container({ id = "chest", name = "chest", portable = false })
  chest:put(entities.Item({ id = "key", name = "key" }))
  world.place(w, "hall", chest)
  return commands.new_state(w)
end

local function play(state)
  commands.run(state, { verb = "take", object = "key" })
  commands.run(state, { verb = "take", object = "lamp" })
  commands.run(state, { verb = "put", object = "lamp", prep = "in", target = "chest" })
  commands.run(state, { verb = "go", object = "north" })
  state.flags.met_cook = true
end

local function ids(list)
  local out = {}
  for i, e in ipairs(list) do out[i] = e.id end
  return out
end

t:scenario("c49_snapshot_and_restore_put_everything_back", function()
  local state = game()
  play(state)
  local snap = save.snapshot(state)
  H.expect(snap, {
    room = "kitchen", turns = 4, flags = { met_cook = true }, inventory = { "key" },
    rooms = { hall = { "chest" }, kitchen = {} }, containers = { chest = { "lamp" } },
  }, "the snapshot")
  local fresh = game()
  H.expect(save.restore(fresh, save.deserialize(save.serialize(snap))), true, "restore")
  H.expect({ fresh.room, fresh.turns, fresh.flags, ids(fresh.inventory) }, { "kitchen", 4, { met_cook = true }, { "key" } }, "room, turns, flags and inventory")
  H.expect({ ids(fresh.world.rooms.hall.items), ids(fresh.world.rooms.hall.items[1].contents) }, { { "chest" }, { "lamp" } }, "the hall and the chest")
  H.check(fresh.inventory[1].matches ~= nil, "the inventory should hold the game's own entities, not plain tables")
  local before = save.snapshot(fresh)
  local ok, err = save.restore(fresh, { room = "hall", inventory = { "unicorn" } })
  H.expect({ ok, err }, { nil, "unknown item 'unicorn' in the save" }, "restoring an unknown item")
  H.expect(save.snapshot(fresh), before, "a failed restore changes nothing")
end)

t:scenario("c50_save_and_load_go_through_a_file", function()
  local path = temp_path("adventure-save")
  local state = game()
  play(state)
  H.expect(save.save(state, path), true, "save")
  local file = assert(io.open(path))
  local text = file:read("a")
  file:close()
  H.check(text:sub(1, 7) == "return ", "the file should hold serialize's output, got: " .. text:sub(1, 40))
  local fresh = game()
  H.expect(save.load(fresh, path), true, "load")
  H.expect(save.snapshot(fresh), save.snapshot(state), "the loaded state")
  os.remove(path)
  local ok, err = save.load(fresh, path)
  H.check(ok == nil and type(err) == "string", "loading a missing file should return nil, err")
end)

t:finish()
