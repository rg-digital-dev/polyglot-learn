-- Part of the Deeper Lua module (tracks/lua/extras/account-classes). Don't edit
-- this file: `learn` overwrites it. Your code goes in account_classes.lua.

-- Resolve modules relative to this file, wherever lua is started from.
local dir = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
package.path = dir .. "/?.lua;" .. dir .. "/../?.lua;" .. package.path

local H = require("harness")
local M = require("account_classes")

local t = H.new()

t:scenario("c01_new_sets_owner_and_balance", function()
  local a = M.Account.new("ann", 100)
  H.expect(a.owner, "ann", "a.owner")
  H.expect(a.balance, 100, "a.balance")
  H.expect(M.Account.new("bob").balance, 0, "balance when none is given")
end)

t:scenario("c02_deposit_adds_and_returns_the_new_balance", function()
  local a = M.Account.new("ann", 100)
  H.expect(a:deposit(50), 150, "a:deposit(50)")
  H.expect(a:deposit(1), 151, "a:deposit(1)")
  H.expect(a.balance, 151, "a.balance")
end)

t:scenario("c03_methods_live_on_the_class_not_the_instance", function()
  local a = M.Account.new("ann", 100)
  H.check(getmetatable(a) == M.Account, "the metatable of an Account should be M.Account")
  H.check(rawget(a, "deposit") == nil, "the instance has its own deposit field: let __index find it on M.Account")
  H.expect(a:withdraw(30), 70, "a:withdraw(30)")
end)

t:returns_error("c04_withdrawing_too_much_returns_nil_err", function()
  return M.Account.new("ann", 100):withdraw(101)
end)

t:returns_error("c05_a_deposit_of_zero_or_less_returns_nil_err", function()
  local a = M.Account.new("ann", 100)
  local v, err = a:deposit(-5)
  if v ~= nil or err == nil then return v, err end
  return a:deposit(0)
end)

t:scenario("c06_a_failed_withdraw_leaves_the_balance_unchanged", function()
  local a = M.Account.new("ann", 100)
  local v, err = a:withdraw(500)
  H.check(v == nil and err ~= nil, "a:withdraw(500) should return nil plus an error")
  v, err = a:withdraw(-1)
  H.check(v == nil and err ~= nil, "a:withdraw(-1) should return nil plus an error")
  H.expect(a.balance, 100, "a.balance")
end)

t:scenario("c07_a_savings_account_inherits_deposit_from_account", function()
  local s = M.SavingsAccount.new("cy", 1000, 2)
  H.check(getmetatable(s) == M.SavingsAccount, "the metatable of a SavingsAccount should be M.SavingsAccount")
  H.check(rawget(M.SavingsAccount, "deposit") == nil,
    "M.SavingsAccount has its own deposit: inherit it through a metatable instead of copying it")
  H.expect(s:deposit(500), 1500, "s:deposit(500)")
  H.expect({ s.owner, s.rate }, { "cy", 2 }, "{ s.owner, s.rate }")
end)

t:scenario("c08_add_interest_adds_whole_cents_rounded_down", function()
  H.expect(M.SavingsAccount.new("cy", 1000, 2):add_interest(), 1020, "1000 at 2%")
  local s = M.SavingsAccount.new("di", 999, 3)
  H.expect(s:add_interest(), 1028, "999 at 3%")
  H.expect(math.type(s.balance), "integer", "math.type(balance)")
end)

t:scenario("c09_a_savings_withdraw_keeps_the_minimum_balance", function()
  local s = M.SavingsAccount.new("cy", 1000, 2)
  H.expect(s:withdraw(900), 100, "s:withdraw(900)")
  local v, err = s:withdraw(1)
  H.check(v == nil and err ~= nil, "s:withdraw(1) below the minimum should return nil plus an error")
  v, err = s:withdraw(-50)
  H.check(v == nil and err ~= nil, "s:withdraw(-50) should still return nil plus an error")
  H.expect(s.balance, 100, "s.balance")
end)

t:scenario("c10_describe_is_overridden_and_calls_the_parent", function()
  H.expect(M.Account.new("ann", 100):describe(), "ann: 100", "Account describe")
  local s = M.SavingsAccount.new("cy", 1000, 2)
  H.expect(s:describe(), "cy: 1000 (savings 2%)", "SavingsAccount describe")
  -- Swap the parent's describe for a moment: the savings version must use it.
  local original = M.Account.describe
  M.Account.describe = function() return "PARENT" end
  local ok, got = pcall(s.describe, s)
  M.Account.describe = original
  H.check(ok, "describe raised an error: " .. tostring(got))
  H.expect(got, "PARENT (savings 2%)", "describe with a replaced Account:describe")
end)

t:finish()
