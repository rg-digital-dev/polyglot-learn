local M = {}

--- A 2D vector. Operators work on it through the metamethods below.
--- @class Vec
--- @field x number
--- @field y number
M.Vec = {}
M.Vec.__index = M.Vec

--- Provided: create a vector.
--- @return Vec
function M.Vec.new(x, y)
  return setmetatable({ x = x, y = y }, M.Vec)
end

--- a + b: a new vector.
function M.Vec.__add(a, b)
  error("TODO: implement Vec.__add")
end

--- a - b: a new vector.
function M.Vec.__sub(a, b)
  error("TODO: implement Vec.__sub")
end

--- -v: a new vector with both components negated.
function M.Vec.__unm(v)
  error("TODO: implement Vec.__unm")
end

--- v * n and n * v scale the vector (a new vector); v * w is the dot product
--- (a number). Either argument may be the number.
function M.Vec.__mul(a, b)
  error("TODO: implement Vec.__mul")
end

--- a == b when both components are equal.
function M.Vec.__eq(a, b)
  error("TODO: implement Vec.__eq")
end

--- a < b when a is shorter than b.
function M.Vec.__lt(a, b)
  error("TODO: implement Vec.__lt")
end

--- a <= b when a is not longer than b.
function M.Vec.__le(a, b)
  error("TODO: implement Vec.__le")
end

--- tostring(v): "(x, y)", for example "(1, -2)".
function M.Vec.__tostring(v)
  error("TODO: implement Vec.__tostring")
end

--- The Euclidean length: the square root of x*x + y*y.
--- @return number
function M.Vec:length()
  error("TODO: implement Vec:length")
end

return M
