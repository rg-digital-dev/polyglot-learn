-- The game: every module wired together behind one object.

local world = require("world")
local parser = require("parser")
local entities = require("entities")
local commands = require("commands")
local events = require("events")
local scripts = require("scripts")
local save = require("save")

local M = {}

--- The class each item `kind` in a world definition makes.
M.KINDS = { thing = entities.Entity, item = entities.Item, container = entities.Container, npc = entities.Npc }

local Game = {}
Game.__index = Game
M.Game = Game

--- A game from a world definition:
---   {
---     title = "...", intro = "...", start = "beach", save_path = "adventure.sav",
---     rooms = { beach = { name = ..., description = ..., exits = { north = "path" } }, ... },
---     items = { { id = "lamp", kind = "item", name = "brass lamp", room = "beach" },
---               { id = "key", kind = "item", name = "key", inside = "chest" }, ... },
---     scripts = { keeper = function(say, ask, game) ... end },   -- for npcs: talk = "keeper"
---     setup = function(game) ... end,                            -- called last
---   }
--- Every item is in a `room` or `inside` a container (even a closed one);
--- `kind` defaults to "item". An item with `talk` gets that script as its
--- `script` field.
--- The game is { def, world, state (commands.new_state), emitter, scheduler,
--- conversation = nil, over = false, won = false, save_path }. Its state
--- gets five more verbs: talk, save and load (to save_path), quit and help
--- ("Verbs: drop, examine, ...", parser.VERBS sorted).
--- @return table|nil game
--- @return string|nil err  for a broken definition: a bad room or exit, an
---   unknown kind, an item in an unknown room or container, a duplicate id,
---   an unknown script
function M.new(def)
  error("TODO: implement game.new")
end

--- Listen for "before <verb>" (a listener returning false, reason blocks the
--- command) or "after <verb>"; listeners get (game, command). Returns `off`.
function Game:on(event, fn)
  error("TODO: implement Game:on")
end

--- Run fn(game) as a background script, a step per turn (see scripts.wait).
function Game:spawn(name, fn)
  error("TODO: implement Game:spawn")
end

--- Add a line to the output of the current turn, after the command's own text.
function Game:say(text)
  error("TODO: implement Game:say")
end

--- The entity with this id, wherever it is. @return table|nil
function Game:item(id)
  return save.index(self.state)[id]
end

--- Whether the player carries the item with this id.
function Game:has(id)
  for _, entity in ipairs(self.state.inventory) do
    if entity.id == id then
      return true
    end
  end
  return false
end

--- Start talking to someone here: their script runs as a conversation,
--- called as script(say, ask, game), and the next inputs answer it until it
--- ends. Returns its first text.
--- Errors: "Talk to whom?", commands.find's error, "The rake has nothing to say."
function Game:talk(word)
  error("TODO: implement Game:talk")
end

--- Save the game state to a file. @return boolean|nil ok, string|nil err
function Game:save(path)
  return save.save(self.state, path)
end

--- Load the game state from a file. @return boolean|nil ok, string|nil err
function Game:load(path)
  return save.load(self.state, path)
end

--- The title, the intro and the first room, separated by blank lines.
function Game:intro()
  error("TODO: implement Game:intro")
end

--- One turn: the player's line in, the text to show out.
--- After quit it is always "The game is over.". In a conversation the line
--- is its answer. Otherwise it is parsed, the "before <verb>" listeners may
--- block it, the command runs, and if it worked the "after <verb>" listeners
--- run and the background scripts take a step; a script's error is shown as
--- "(script error: name: message)". Lines from game:say come after the
--- command's own text (or error), one per line.
function Game:input(line)
  error("TODO: implement Game:input")
end

return M
