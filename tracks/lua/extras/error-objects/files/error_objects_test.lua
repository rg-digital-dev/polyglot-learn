-- Part of the Deeper Lua module (tracks/lua/extras/error-objects). Don't edit
-- this file: `learn` overwrites it. Your code goes in error_objects.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local M = require("error_objects")

local t = H.new()

-- Call fn(...), which must raise, and return what it raised. The stub's own
-- "TODO: implement" error doesn't count as the expected error.
local function caught(fn, ...)
  local ok, err = pcall(fn, ...)
  H.check(not ok, "expected an error, but the call returned normally")
  H.check(not (type(err) == "string" and err:find("TODO: implement", 1, true)), "raised the stub's error: " .. tostring(err))
  return err
end

local function expect_error_object(err, code, what)
  H.check(type(err) == "table", what .. ": expected an error object (a table), got " .. type(err) .. " " .. tostring(err))
  H.expect(err.code, code, what .. " .code")
  H.check(type(err.message) == "string" and err.message ~= "", what .. ": .message should be a non-empty string")
end

t:equals("c01_check_age_returns_a_valid_age", function()
  return { M.check_age(30), M.check_age(0), M.check_age(150) }
end, { 30, 0, 150 })

t:scenario("c02_check_age_raises_a_table_with_code_and_message", function()
  expect_error_object(caught(M.check_age, 151), "out_of_range", "check_age(151)")
  expect_error_object(caught(M.check_age, -1), "out_of_range", "check_age(-1)")
end)

t:scenario("c03_check_age_rejects_values_that_are_not_integers", function()
  expect_error_object(caught(M.check_age, 30.5), "not_an_integer", "check_age(30.5)")
  expect_error_object(caught(M.check_age, "30"), "not_an_integer", "check_age(\"30\")")
  expect_error_object(caught(M.check_age, nil), "not_an_integer", "check_age(nil)")
end)

t:scenario("c04_try_returns_the_value_when_nothing_is_raised", function()
  H.expect(M.try(M.check_age, 42), 42, "try(check_age, 42)")
  H.expect(M.try(function(a, b) return a + b end, 2, 3), 5, "try passes the extra arguments on")
end)

t:scenario("c05_try_turns_a_raised_error_object_into_nil_err", function()
  local v, err = M.try(M.check_age, 200)
  H.expect(v, nil, "the first result of try(check_age, 200)")
  expect_error_object(err, "out_of_range", "the error from try(check_age, 200)")
end)

t:scenario("c06_try_adds_a_traceback", function()
  local _, err = M.try(M.check_age, 200)
  H.check(type(err) == "table", "try(check_age, 200) should return nil plus an error object")
  H.check(type(err.traceback) == "string" and err.traceback:find("stack traceback", 1, true),
    "err.traceback should hold the text of debug.traceback()")
  H.check(err.traceback:find("check_age", 1, true),
    "the traceback should include check_age, where the error was raised: take it inside the xpcall handler")
end)

t:scenario("c07_try_wraps_a_string_error_as_an_internal_error", function()
  local v, err = M.try(function() error("disk full", 0) end)
  H.expect(v, nil, "the first result")
  H.check(type(err) == "table", "a string error should come back as an error object, got " .. tostring(err))
  H.expect({ err.code, err.message }, { "internal", "disk full" }, "{ code, message }")
  H.check(type(err.traceback) == "string", "err.traceback should be set for string errors too")
end)

t:scenario("c08_must_returns_the_value_or_raises_the_error", function()
  H.expect(M.must(7, nil), 7, "must(7, nil)")
  local e = { code = "out_of_range", message = "too old" }
  H.check(rawequal(caught(M.must, nil, e), e), "must(nil, e) should raise e itself")
  expect_error_object(caught(function() return M.must(M.try(M.check_age, -5)) end),
    "out_of_range", "must(try(check_age, -5))")
end)

t:scenario("c09_require_field_blames_the_caller", function()
  H.expect(M.require_field({ id = 3 }, "id"), 3, "require_field({ id = 3 }, \"id\")")
  local err = caught(function()
    local v = M.require_field({}, "id")  -- not `return M.require_field(...)`: a tail call leaves no caller line
    return v
  end)
  H.check(type(err) == "string", "expected a string error, got " .. type(err))
  H.check(err:find("missing field 'id'", 1, true), "unexpected message: " .. err)
  H.check(err:find("error_objects_test.lua:%d+: missing field") ~= nil,
    "the position should point at the caller in error_objects_test.lua (use error's level argument), got: " .. err)
end)

t:scenario("c10_with_context_adds_context_and_rethrows", function()
  H.expect(M.with_context(function() return 7 end, "row 3"), 7, "with_context returns fn's result")
  local original = caught(M.check_age, -1)
  local err = caught(M.with_context, function() return M.check_age(-1) end, "row 3")
  expect_error_object(err, "out_of_range", "rethrown error")
  H.expect(err.message, "row 3: " .. tostring(original.message), "rethrown message")
  -- Any other error value comes through untouched: no extra position in front.
  local function boom() error("boom") end
  local plain = caught(boom)
  H.expect(caught(M.with_context, boom, "row 3"), plain, "a rethrown string error")
end)

t:finish()
