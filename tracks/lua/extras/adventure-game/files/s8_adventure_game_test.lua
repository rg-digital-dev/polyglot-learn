-- Stage 8 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in game.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local game = require("game")
local scripts = require("scripts")

local t = H.new()

-- A file name in the system's temporary folder.
local function temp_path(name)
  local base = os.getenv("TMPDIR") or os.getenv("TEMP") or os.getenv("TMP") or "/tmp"
  return base .. "/" .. name .. "-" .. tostring(os.time()) .. "-" .. tostring(math.random(1e9))
end

-- A small world: a garden with a gnome, a shed with a box holding a trowel.
local function def()
  return {
    title = "GARDEN",
    intro = "A sunny afternoon.",
    start = "garden",
    rooms = {
      garden = { name = "Garden", description = "Flowers everywhere.", exits = { east = "shed" } },
      shed = { name = "Shed", description = "Cobwebs.", exits = { west = "garden" } },
    },
    items = {
      { id = "gnome", kind = "npc", name = "gnome", room = "garden", talk = "gnome" },
      { id = "rake", name = "rake", room = "garden" },
      { id = "box", kind = "container", name = "box", room = "shed", portable = false, open = false },
      { id = "trowel", name = "trowel", inside = "box" },
      { id = "bench", kind = "thing", name = "bench", room = "garden" },
    },
    scripts = {
      gnome = function(say, ask, g)
        local answer = ask("\"Want my secret?\"", { "yes", "no" })
        if answer == "yes" then
          g.state.flags.secret = true
          say("\"The box opens if you knock.\"")
        end
        return answer
      end,
    },
  }
end

local function new(d)
  local g, err = game.new(d or def())
  H.check(g ~= nil, "game.new returned nil, " .. tostring(err))
  return g
end

local function ids(list)
  local out = {}
  for i, e in ipairs(list) do out[i] = e.id end
  return out
end

t:scenario("c51_new_builds_the_world_the_items_and_the_player", function()
  local g = new()
  H.expect(g.state.room, "garden", "the player's room")
  H.expect(ids(g.world.rooms.garden.items), { "gnome", "rake", "bench" }, "the garden's items")
  local box = g:item("box")
  H.check(box ~= nil and getmetatable(box) == require("entities").Container, "the box should be a Container")
  H.expect({ box.open, ids(box.contents) }, { false, { "trowel" } }, "the closed box starts with the trowel")
  H.check(getmetatable(g:item("gnome")) == require("entities").Npc and getmetatable(g:item("rake")) == require("entities").Item, "kinds: npc makes an Npc, no kind makes an Item")
  H.check(getmetatable(g:item("bench")) == require("entities").Entity, "kind thing makes an Entity")
  H.expect(g:intro(), "GARDEN\n\nA sunny afternoon.\n\nGarden\nFlowers everywhere.\nExits: east.\nYou see: gnome, rake, bench.", "intro")
  H.expect({ g.over, g.won }, { false, false }, "over and won")
end)

t:scenario("c52_new_returns_nil_and_an_error_for_a_broken_definition", function()
  local function broken(change, mentions)
    local d = def()
    change(d)
    local g, err = game.new(d)
    H.check(g == nil and type(err) == "string" and err:find(mentions, 1, true), "expected nil and an error mentioning " .. string.format("%q", mentions) .. ", got " .. tostring(g) .. ", " .. tostring(err))
  end
  broken(function(d) d.rooms.shed.exits.up = "loft" end, "loft")
  broken(function(d) d.items[2].kind = "vegetable" end, "vegetable")
  broken(function(d) d.items[2].room = "pond" end, "pond")
  broken(function(d) d.items[4].inside = "rake" end, "rake")
  broken(function(d) d.items[5].id = "rake" end, "rake")
  broken(function(d) d.items[1].talk = "elf" end, "elf")
end)

t:scenario("c53_input_parses_runs_and_answers", function()
  local g = new()
  H.expect(g:input("take the rake"), "Taken.", "take the rake")
  H.expect(g:input("i"), "You are carrying a rake.", "i")
  H.expect(g:input("dance"), 'I don\'t know the word "dance".', "dance")
  H.expect(g:input("take bench"), "You can't take the bench.", "take bench")
  H.expect(g:input("e"), "Shed\nCobwebs.\nExits: west.\nYou see: box.", "e")
  H.expect(g:input("help"):match("^Verbs: "), "Verbs: ", "help")
  H.expect(g.state.turns, 4, "turns")
end)

t:scenario("c54_before_listeners_can_block_and_after_listeners_follow", function()
  local g = new()
  local log = {}
  g:on("before go", function(gm, command)
    H.check(gm == g, "listeners get the game first")
    if not gm:has("rake") then return false, "You feel you need a rake." end
  end)
  g:on("after take", function(gm, command)
    log[#log + 1] = command.object
    gm:say("The gnome nods.")
  end)
  H.expect(g:input("e"), "You feel you need a rake.", "e without the rake")
  H.expect(g:input("take rake"), "Taken.\nThe gnome nods.", "take rake")
  H.expect(g:input("e"):match("^Shed"), "Shed", "e with the rake")
  H.expect(g:input("take box"), "You can't take the box.", "a failed take has no after")
  H.expect(log, { "rake" }, "after take ran for")
end)

t:scenario("c55_talk_starts_a_conversation_that_takes_the_next_inputs", function()
  local g = new()
  H.expect(g:input("talk to gnome"), "\"Want my secret?\"\n1) yes  2) no", "talk to gnome")
  H.check(g.conversation ~= nil, "game.conversation should be set while talking")
  H.expect(g:input("maybe"), "Please answer with one of: yes, no.", "an answer that isn't a choice")
  H.expect(g:input("yes"), "\"The box opens if you knock.\"", "yes")
  H.expect({ g.conversation, g.state.flags.secret }, { nil, true }, "conversation and flag afterwards")
  H.expect(g:input("talk to rake"), "The rake has nothing to say.", "talk to rake")
  H.expect(g:input("talk to troll"), "There is no troll here.", "talk to troll")
end)

t:scenario("c56_spawned_scripts_take_a_step_after_each_turn", function()
  local g = new()
  g:spawn("bee", function(gm)
    while true do
      scripts.wait(2)
      gm:say("A bee buzzes past.")
    end
  end)
  g:spawn("oops", function() error("broken script", 0) end)
  local function bee(out) return out:match("\nA bee buzzes past.$") ~= nil end
  local first = g:input("look")
  H.check(first:match("\n%(script error: oops: broken script%)$"), "the first turn should end by reporting the broken script, got: " .. first)
  H.expect(bee(g:input("look")), false, "a bee on turn 2")
  H.expect(bee(g:input("look")), true, "a bee on turn 3")
  H.expect(g:input("xyzzy"), 'I don\'t know the word "xyzzy".', "a failed command isn't a turn")
  H.expect(bee(g:input("look")), false, "a bee on turn 4")
  H.expect(bee(g:input("look")), true, "a bee on turn 5")
end)

t:scenario("c57_save_load_and_quit", function()
  local d = def()
  d.save_path = temp_path("adventure-game")
  local g = new(d)
  g:input("take rake")
  g:input("e")
  H.expect(g:input("save"), "Saved.", "save")
  g:input("w")
  g:input("drop rake")
  H.expect(g:input("load"), "Loaded.\nShed\nCobwebs.\nExits: west.\nYou see: box.", "load")
  H.expect({ g.state.room, ids(g.state.inventory) }, { "shed", { "rake" } }, "room and inventory after load")
  os.remove(d.save_path)
  H.expect(g:input("quit"), "Goodbye.", "quit")
  H.expect({ g.over, g:input("look") }, { true, "The game is over." }, "after quit")
end)

t:scenario("c58_the_lighthouse_can_be_won", function()
  local g = new(dofile(dir .. "/lighthouse.lua"))
  local out
  for _, line in ipairs({ "take lamp", "n", "n", "u" }) do out = g:input(line) end
  H.expect(out, "The keeper blocks the stair.", "going up before talking to the keeper")
  for _, line in ipairs({ "talk to keeper", "yes", "d", "take oil", "u", "u" }) do out = g:input(line) end
  H.expect(out:match("^Lamp room") ~= nil, true, "reached the lamp room: " .. tostring(out))
  out = g:input("put oil in mechanism")
  H.check(out:find("You win!", 1, true), "putting the oil in the mechanism should win, got: " .. tostring(out))
  H.expect({ g.won, g.over }, { true, true }, "won and over")
end)

t:finish()
