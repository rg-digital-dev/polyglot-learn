-- Stage 3 of the text adventure project. Don't edit this file: `learn`
-- overwrites it. Your code goes in entities.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local entities = require("entities")
local Entity, Item, Container, Npc = entities.Entity, entities.Item, entities.Container, entities.Npc

local t = H.new()

t:scenario("c15_entity_new_keeps_its_fields_and_needs_an_id_and_a_name", function()
  local statue = Entity.new({ id = "statue", name = "marble statue", heavy = true })
  H.expect({ statue.id, statue.name, statue.heavy, statue.aliases, statue.article }, { "statue", "marble statue", true, {}, "a" }, "the statue's fields")
  H.expect(Entity.new({ id = "owl", name = "owl" }).article, "an", "the article before a vowel")
  H.expect(Entity.new({ id = "u", name = "unicorn", article = "a" }).article, "a", "a given article")
  local ok, err = pcall(Entity.new, { id = "nameless" })
  H.check(not ok, "Entity.new without a name should raise an error")
  H.check(not tostring(err):find("TODO: implement", 1, true), "raised the stub's error: " .. tostring(err))
  -- Level 2 blames the caller, so the message starts with this file's name.
  local _, blamed = pcall(function()
    local e = Entity.new({ name = "no id" }) -- not a tail call, so this line is the caller
    return e
  end)
  H.check(tostring(blamed):find("s3_adventure_entities_test.lua", 1, true), "the error should blame the caller (error level 2), got: " .. tostring(blamed))
end)

t:scenario("c16_tostring_adds_the_article_for_every_class", function()
  H.expect(tostring(Entity.new({ id = "statue", name = "statue" })), "a statue", "tostring of an Entity")
  H.expect(tostring(Item.new({ id = "apple", name = "apple" })), "an apple", "tostring of an Item")
  H.expect(tostring(Container.new({ id = "box", name = "wooden box" })), "a wooden box", "tostring of a Container")
  H.expect(tostring(Npc.new({ id = "keeper", name = "keeper", article = "the" })), "the keeper", "tostring of an Npc")
end)

t:scenario("c17_matches_the_id_the_name_its_last_word_and_aliases_in_any_case", function()
  local lamp = Item.new({ id = "lamp1", name = "brass lamp", aliases = { "lantern" } })
  for _, word in ipairs({ "lamp1", "brass lamp", "lamp", "LAMP", "Lantern" }) do
    H.expect(lamp:matches(word), true, "lamp:matches(\"" .. word .. "\")")
  end
  for _, word in ipairs({ "brass", "lamps", "torch" }) do
    H.expect(lamp:matches(word), false, "lamp:matches(\"" .. word .. "\")")
  end
end)

t:scenario("c18_classes_inherit_through_their_metatables", function()
  local box = Container.new({ id = "box", name = "box" })
  H.check(rawget(box, "matches") == nil and rawget(Container, "matches") == nil and rawget(Item, "matches") == nil, "matches should live on Entity only, found through __index")
  H.check(box.matches == Entity.matches, "a container should find Entity's matches")
  H.expect({ Entity.portable, Item.portable, Npc.portable, box.portable }, { false, true, false, true }, "portable on Entity, Item, Npc and a container")
  H.expect({ Item.weight, box.capacity, box.open }, { 1, 10, true }, "Item.weight, a container's capacity and open")
  H.expect(Item.new({ id = "anvil", name = "anvil", weight = 50 }).weight, 50, "a given weight")
  H.expect({ entities.is_a(box, Container), entities.is_a(box, Item), entities.is_a(box, Entity), entities.is_a(box, Npc) }, { true, true, true, false }, "is_a(box, ...)")
  H.expect(entities.is_a({}, Entity), false, "is_a of a plain table")
  H.check(getmetatable(box) == Container, "a container's metatable should be Container")
end)

t:scenario("c19_classes_can_be_called_like_constructors", function()
  local coin = Item({ id = "coin", name = "coin" })
  H.check(getmetatable(coin) == Item, "Item{...} should make an Item")
  H.expect(tostring(coin), "a coin", "tostring(Item{...})")
  H.check(getmetatable(Container({ id = "bag", name = "bag" })) == Container, "Container{...} should make a Container")
  H.check(getmetatable(Npc({ id = "cat", name = "cat" })) == Npc, "Npc{...} should make an Npc")
end)

t:scenario("c20_a_container_holds_items_up_to_its_capacity_when_open", function()
  local box = Container.new({ id = "box", name = "box", capacity = 3 })
  local key, brick = Item.new({ id = "key", name = "key" }), Item.new({ id = "brick", name = "brick", weight = 2 })
  H.expect(box:put(key), true, "putting a key in the box")
  H.expect(box:put(brick), true, "putting a brick in the box")
  H.expect(box:weight_inside(), 3, "the weight inside")
  H.expect({ box:put(Item.new({ id = "pin", name = "pin" })) }, { nil, "That won't fit in the box." }, "putting in one more")
  H.expect({ box:put(box) }, { nil, "You can't put the box inside itself." }, "putting the box in itself")
  H.check(box:remove("key") == key, "remove should return the key")
  H.expect(box.contents, { brick }, "the contents after removing the key")
  box.open = false
  H.expect({ box:put(key) }, { nil, "The box is closed." }, "putting the key in the closed box")
  H.expect(Container.new({ id = "jar", name = "jar", open = false }).open, false, "a container made closed")
end)

t:scenario("c21_descriptions_and_lists_read_like_english", function()
  H.expect(entities.list({}), "", "list of nothing")
  local key, coin, apple = Item({ id = "key", name = "key" }), Item({ id = "coin", name = "coin" }), Item({ id = "apple", name = "apple" })
  H.expect(entities.list({ key }), "a key", "list of one")
  H.expect(entities.list({ key, coin }), "a key and a coin", "list of two")
  H.expect(entities.list({ key, coin, apple }), "a key, a coin and an apple", "list of three")
  H.expect(Item({ id = "rock", name = "rock" }):describe(), "You see nothing special about the rock.", "an item without a description")
  local chest = Container({ id = "chest", name = "chest", description = "An old chest." })
  H.expect(chest:describe(), "An old chest. It's empty.", "an empty chest")
  chest:put(key)
  chest:put(coin)
  H.expect(chest:describe(), "An old chest. It contains a key and a coin.", "a chest with two things")
  chest.open = false
  H.expect(chest:describe(), "An old chest. It's closed.", "a closed chest")
end)

t:finish()
