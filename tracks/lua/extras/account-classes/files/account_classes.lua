local M = {}

--- A bank account. Amounts are whole cents.
--- @class Account
--- @field owner string
--- @field balance integer
M.Account = {}
M.Account.__index = M.Account

--- Create an account. `balance` is optional and defaults to 0.
--- @param owner string
--- @param balance integer|nil
--- @return Account
function M.Account.new(owner, balance)
  error("TODO: implement Account.new")
end

--- Add `amount` to the balance and return the new balance.
--- @return integer|nil balance
--- @return string|nil err  return nil, "message" if amount <= 0; don't raise
function M.Account:deposit(amount)
  error("TODO: implement Account:deposit")
end

--- Take `amount` out and return the new balance.
--- @return integer|nil balance
--- @return string|nil err  return nil, "message" if amount <= 0 or more than the balance
function M.Account:withdraw(amount)
  error("TODO: implement Account:withdraw")
end

--- "<owner>: <balance>", for example "ann: 100".
--- @return string
function M.Account:describe()
  error("TODO: implement Account:describe")
end

--- An Account that earns interest and keeps a minimum balance.
--- @class SavingsAccount: Account
--- @field rate integer  interest in whole percent
M.SavingsAccount = {}
M.SavingsAccount.__index = M.SavingsAccount
-- TODO: make SavingsAccount fall back to Account for the methods it doesn't
-- define itself (deposit). Don't copy them over.

--- A withdraw may never leave less than this in a savings account.
M.SavingsAccount.MINIMUM = 100

--- @param owner string
--- @param balance integer|nil
--- @param rate integer
--- @return SavingsAccount
function M.SavingsAccount.new(owner, balance, rate)
  error("TODO: implement SavingsAccount.new")
end

--- Add `balance * rate // 100` and return the new balance.
--- @return integer
function M.SavingsAccount:add_interest()
  error("TODO: implement SavingsAccount:add_interest")
end

--- Like Account:withdraw, but also return nil, "message" when the withdraw
--- would leave less than MINIMUM. Reuse Account:withdraw for the rest.
--- @return integer|nil balance
--- @return string|nil err
function M.SavingsAccount:withdraw(amount)
  error("TODO: implement SavingsAccount:withdraw")
end

--- Account's description plus " (savings <rate>%)", for example
--- "cy: 1000 (savings 2%)". Build it from Account:describe.
--- @return string
function M.SavingsAccount:describe()
  error("TODO: implement SavingsAccount:describe")
end

return M
