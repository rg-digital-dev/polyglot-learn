-- A small world to play: `lua main.lua` (or `lua main.lua lighthouse.lua`).
-- A world file is a Lua chunk that returns a definition table; it is code you
-- trust, so unlike a save file it can hold functions.

local wait = require("scripts").wait

return {
  title = "THE LIGHTHOUSE",
  intro = "The storm is coming and the light is out. Somewhere a ship is heading for the rocks.",
  start = "beach",
  rooms = {
    beach = {
      name = "Beach",
      description = "Wet sand and black rocks. A path climbs north.",
      exits = { north = "path" },
    },
    path = {
      name = "Cliff path",
      description = "The wind tugs at you. The lighthouse stands to the north.",
      exits = { south = "beach", north = "base" },
    },
    base = {
      name = "Lighthouse",
      description = "A round room with a spiral stair going up and a trapdoor going down.",
      exits = { south = "path", up = "top", down = "cellar" },
    },
    cellar = {
      name = "Cellar",
      description = "Damp stone walls. It smells of lamp oil.",
      exits = { up = "base" },
    },
    top = {
      name = "Lamp room",
      description = "Glass on every side. The great lamp sits cold and empty.",
      exits = { down = "base" },
    },
  },
  items = {
    { id = "lamp", name = "brass lamp", aliases = { "lantern" }, room = "beach",
      description = "An old brass lamp. It still burns." },
    { id = "keeper", kind = "npc", name = "lighthouse keeper", aliases = { "man" }, room = "base",
      talk = "keeper", description = "An old man in a wet coat, guarding the stair." },
    { id = "chest", kind = "container", name = "sea chest", room = "cellar", portable = false,
      description = "A chest that came ashore long ago." },
    { id = "oil", name = "can of oil", aliases = { "can" }, inside = "chest", weight = 2 },
    { id = "mechanism", kind = "container", name = "lamp mechanism", room = "top", portable = false,
      capacity = 2, description = "The reservoir that feeds the great lamp." },
  },
  scripts = {
    keeper = function(say, ask, game)
      say("\"Who goes there?\" the keeper asks.")
      local answer = ask("\"Have you come to light the lamp?\"", { "yes", "no" })
      if answer == "yes" then
        game.state.flags.keeper_ok = true
        say("\"Then you'll need oil. There's a can in the cellar. Go on up when you have it.\"")
        return "helped"
      end
      say("\"Then off with you.\"")
      return "refused"
    end,
  },
  setup = function(game)
    game:on("before go", function(g, command)
      local room = g.state.room
      if room == "base" and command.object == "down" and not g:has("lamp") then
        return false, "It's too dark to go down without a light."
      end
      if room == "base" and command.object == "up" and not g.state.flags.keeper_ok then
        return false, "The keeper blocks the stair."
      end
    end)
    game:on("after put", function(g)
      local mechanism = g:item("mechanism")
      for _, item in ipairs(mechanism.contents) do
        if item.id == "oil" then
          g:say("The great lamp roars into life and its beam sweeps across the sea. The ship turns away from the rocks. You win!")
          g.won, g.over = true, true
        end
      end
    end)
    game:spawn("tide", function(g)
      while true do
        wait(4)
        if g.state.room == "beach" then
          g:say("A wave washes over your feet.")
        end
      end
    end)
  end,
}
