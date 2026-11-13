-- Stage 2 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in parser.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local parser = require("parser")

local t = H.new()

local function parses(line, expected)
  local command, err = parser.parse(line)
  H.check(command ~= nil, "parse(" .. string.format("%q", line) .. ") returned nil, " .. tostring(err))
  H.expect(command, expected, "parse(" .. string.format("%q", line) .. ")")
end

t:scenario("c09_words_lowercases_and_drops_punctuation_and_articles", function()
  H.expect(parser.words("Take the Brass Lamp!"), { "take", "brass", "lamp" }, "words(\"Take the Brass Lamp!\")")
  H.expect(parser.words("  look,   at an  old-fashioned clock... "), { "look", "at", "old-fashioned", "clock" }, "words with extra spaces")
  H.expect(parser.words("give a coin to the keeper's dog"), { "give", "coin", "to", "keeper's", "dog" }, "words with an apostrophe")
  H.expect(parser.words("   "), {}, "words of a blank line")
end)

t:scenario("c10_a_direction_alone_means_go", function()
  parses("n", { verb = "go", object = "north" })
  parses("North", { verb = "go", object = "north" })
  parses("go west", { verb = "go", object = "west" })
  parses("walk d", { verb = "go", object = "down" })
end)

t:scenario("c11_synonyms_and_two_word_verbs_become_one_verb", function()
  parses("get lamp", { verb = "take", object = "lamp" })
  parses("pick up the lamp", { verb = "take", object = "lamp" })
  parses("look at the lamp", { verb = "examine", object = "lamp" })
  parses("x lamp", { verb = "examine", object = "lamp" })
  parses("talk to the keeper", { verb = "talk", object = "keeper" })
  parses("i", { verb = "inventory" })
  parses("look", { verb = "look" })
end)

t:scenario("c12_the_object_can_be_several_words", function()
  parses("take the brass lamp", { verb = "take", object = "brass lamp" })
  parses("examine an old sea chest", { verb = "examine", object = "old sea chest" })
end)

t:scenario("c13_a_preposition_splits_the_object_and_the_target", function()
  parses("put the lamp into the box", { verb = "put", object = "lamp", prep = "in", target = "box" })
  parses("put can of oil in lamp mechanism", { verb = "put", object = "can of oil", prep = "in", target = "lamp mechanism" })
  parses("put coin on table", { verb = "put", object = "coin", prep = "on", target = "table" })
end)

t:scenario("c14_empty_input_and_unknown_words_return_nil_and_an_error", function()
  H.expect({ parser.parse("") }, { nil, "Say something." }, "parse(\"\")")
  H.expect({ parser.parse("the the") }, { nil, "Say something." }, "parse(\"the the\")")
  H.expect({ parser.parse("xyzzy now") }, { nil, 'I don\'t know the word "xyzzy".' }, "parse(\"xyzzy now\")")
  H.expect({ parser.parse("Dance!") }, { nil, 'I don\'t know the word "dance".' }, "parse(\"Dance!\")")
end)

t:finish()
