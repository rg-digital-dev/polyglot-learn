-- Turns what the player types into a command table.

local M = {}

--- The verbs the game understands.
M.VERBS = {
  look = true, go = true, take = true, drop = true, inventory = true, examine = true,
  put = true, talk = true, save = true, load = true, quit = true, help = true,
}

--- One-word synonyms for a verb.
M.SYNONYMS = {
  get = "take", grab = "take", l = "look", x = "examine", i = "inventory", inv = "inventory",
  q = "quit", walk = "go", leave = "drop", speak = "talk",
}

--- Two-word verbs: "pick up the lamp" is take.
M.PHRASES = {
  ["pick up"] = "take", ["look at"] = "examine", ["talk to"] = "talk", ["speak to"] = "talk",
  ["put down"] = "drop",
}

--- Short and long names for each direction.
M.DIRECTIONS = {
  n = "north", s = "south", e = "east", w = "west", u = "up", d = "down",
  north = "north", south = "south", east = "east", west = "west", up = "up", down = "down",
}

--- Words between an object and its target, and what they mean.
M.PREPOSITIONS = { ["in"] = "in", into = "in", inside = "in", on = "on", onto = "on", to = "to", with = "with", at = "at" }

--- The words of a line: lower case, punctuation other than ' and - dropped,
--- and without the articles "the", "a" and "an".
--- @return string[]
function M.words(line)
  error("TODO: implement parser.words")
end

--- Parse a line into { verb = ..., object = ..., prep = ..., target = ... }.
--- "n" and "go north" both give { verb = "go", object = "north" }.
--- "put the lamp into the box" gives { verb = "put", object = "lamp", prep = "in", target = "box" }.
--- @return table|nil command
--- @return string|nil err  "Say something." for an empty line,
---   'I don't know the word "xyzzy".' for an unknown verb
function M.parse(line)
  error("TODO: implement parser.parse")
end

return M
