-- Part of the Deeper Lua module (tracks/lua/extras/vector-ops). Don't edit
-- this file: `learn` overwrites it. Your code goes in vector_ops.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local M = require("vector_ops")
local V = M.Vec.new

local t = H.new()

-- The result must be a new vector with these components.
local function expect_vec(got, x, y, what)
  H.check(type(got) == "table" and getmetatable(got) == M.Vec, what .. ": expected a vector made with M.Vec.new")
  H.expect({ got.x, got.y }, { x, y }, what .. " {x, y}")
end

t:scenario("c01_adding_two_vectors", function()
  local a, b = V(1, 2), V(3, 4)
  expect_vec(a + b, 4, 6, "(1, 2) + (3, 4)")
  expect_vec(a, 1, 2, "a after a + b")
  expect_vec(a + b + V(-4, 0), 0, 6, "(1, 2) + (3, 4) + (-4, 0)")
end)

t:scenario("c02_subtracting_two_vectors", function()
  local a, b = V(5, 1), V(2, 4)
  expect_vec(a - b, 3, -3, "(5, 1) - (2, 4)")
  expect_vec(b - a, -3, 3, "(2, 4) - (5, 1)")
end)

t:scenario("c03_unary_minus_flips_both_components", function()
  expect_vec(-V(2, -7), -2, 7, "-(2, -7)")
  expect_vec(-(-V(1, 1)), 1, 1, "-(-(1, 1))")
end)

t:scenario("c04_vector_times_a_number", function()
  expect_vec(V(1, -2) * 3, 3, -6, "(1, -2) * 3")
  expect_vec(V(4, 5) * 0, 0, 0, "(4, 5) * 0")
end)

t:scenario("c05_number_times_a_vector", function()
  expect_vec(3 * V(1, -2), 3, -6, "3 * (1, -2)")
  expect_vec(2 * (V(1, 0) + V(0, 1)), 2, 2, "2 * ((1, 0) + (0, 1))")
end)

t:scenario("c06_vector_times_vector_is_the_dot_product", function()
  H.expect(V(1, 2) * V(3, 4), 11, "(1, 2) * (3, 4)")
  H.expect(V(1, 0) * V(0, 5), 0, "(1, 0) * (0, 5)")
end)

t:scenario("c07_equal_components_compare_equal", function()
  H.check(V(1, 2) == V(1, 2), "(1, 2) == (1, 2) should be true")
  H.check(V(1, 2) ~= V(2, 1), "(1, 2) ~= (2, 1) should be true")
  H.check(V(3, 4) - V(1, 1) == V(2, 3), "(3, 4) - (1, 1) == (2, 3) should be true")
end)

t:scenario("c08_tostring_shows_the_components", function()
  H.expect(tostring(V(1, -2)), "(1, -2)", "tostring((1, -2))")
  H.expect(string.format("at %s", V(0, 7)), "at (0, 7)", "string.format with %s")
end)

t:scenario("c09_length_is_the_euclidean_length", function()
  H.expect(V(3, 4):length(), 5.0, "(3, 4):length()")
  H.expect(V(0, -2):length(), 2.0, "(0, -2):length()")
  H.expect(V(0, 0):length(), 0.0, "(0, 0):length()")
end)

t:scenario("c10_shorter_vectors_sort_first", function()
  H.check(V(1, 1) < V(0, 3), "(1, 1) < (0, 3) should be true")
  H.check(not (V(3, 0) < V(0, 3)), "(3, 0) < (0, 3) should be false: same length")
  H.check(V(3, 0) <= V(0, 3), "(3, 0) <= (0, 3) should be true: same length")
  H.check(V(0, 5) > V(4, 0), "(0, 5) > (4, 0) should be true")
  local list = { V(0, 5), V(1, 0), V(2, 2) }
  table.sort(list)
  H.expect(tostring(list[1]) .. " " .. tostring(list[2]) .. " " .. tostring(list[3]),
    "(1, 0) (2, 2) (0, 5)", "table.sort with no comparator")
end)

t:finish()
