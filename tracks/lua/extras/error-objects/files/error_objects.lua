local M = {}

--- An error object: a table raised with error() and caught by pcall/xpcall.
--- @class ErrorObject
--- @field code string               "out_of_range", "not_an_integer" or "internal"
--- @field message string
--- @field traceback string|nil      set by M.try

--- Return `age` when it is an integer from 0 to 150. Otherwise raise an error
--- object: code "not_an_integer" (including non-numbers) or "out_of_range".
--- @param age any
--- @return integer
function M.check_age(age)
  error("TODO: implement check_age")
end

--- Call fn(...) and return its first result. If it raises, return nil plus an
--- error object instead. A raised string becomes { code = "internal",
--- message = <the string> }. Either way, set err.traceback to a stack
--- traceback taken where the error happened (xpcall + debug.traceback).
--- @return any|nil result
--- @return ErrorObject|nil err
function M.try(fn, ...)
  error("TODO: implement try")
end

--- The reverse of try: return `value` when it is truthy, otherwise raise `err`
--- unchanged. Lets you write `M.must(M.try(...))`.
function M.must(value, err)
  error("TODO: implement must")
end

--- Return t[name]. When it is nil, raise the string "missing field '<name>'"
--- with the position of the code that called require_field, not of this file.
--- @param t table
--- @param name string
function M.require_field(t, name)
  error("TODO: implement require_field")
end

--- Call fn() and return its first result. If it raises an error object, raise a
--- new one with the same code and the message "<context>: <message>". Rethrow
--- any other error value exactly as it was.
--- @param fn fun(): any
--- @param context string
function M.with_context(fn, context)
  error("TODO: implement with_context")
end

return M
