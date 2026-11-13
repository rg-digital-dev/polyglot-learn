-- Play: `lua main.lua [world file]`, from this folder or any other.

local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. package.path

local game_module = require("game")

local path = arg[1] or (dir .. "/lighthouse.lua")
local def = dofile(path) -- a world file is trusted code, unlike a save
local game, err = game_module.new(def)
if not game then
  io.stderr:write(path, ": ", err, "\n")
  os.exit(1)
end

print(game:intro())
while not game.over do
  io.write("\n> ")
  io.stdout:flush()
  local line = io.read("l")
  if not line then
    break
  end
  print(game:input(line))
end
