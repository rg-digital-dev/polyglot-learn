# Types and Annotations in Lua

Lua has no type annotations and no type checker. Values carry their type, and variables do not: `local x = 1` can later hold a string. There are eight types: `nil`, `boolean`, `number`, `string`, `table`, `function`, `userdata` and `thread`. `type(v)` returns the name as a string. A `number` is either an integer or a float, and `math.type(v)` tells you which. Nothing checks a value until an operation uses it, so a wrong type shows up as a runtime error on that line. Strings that look like numbers convert in arithmetic, and numbers convert to strings under `..`.

```lua
print(type(1), type("a"), type(nil), type({}), type(print))
--> number   string   nil   table   function
print(math.type(1), math.type(1.0), math.type("1"))  --> integer   float   nil
print(1 == 1.0, 7 // 2, 7 / 2)                       --> true   3   3.5
print("10" + 1, 10 .. 20)                            --> 11   1020
print(pcall(function() return {} .. "x" end))
--> false   ...: attempt to concatenate a table value
print(tonumber("42"), tonumber("abc"), tostring(12)) --> 42   nil   12
```

<!-- from:python -->
## Coming from Python
- Lua has no `int` or `float` type name: both are `number`. Use `math.type` to tell them apart.
- `type(x)` gives a string, not a class. Compare with `type(x) == "string"`.
- `nil` is Python's `None`, but reading a missing table key gives `nil` instead of raising.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Nothing is checked before you run the code, and there is no `tsc` to run. The `--- @param` comments in the stubs are notes for you and your editor.
- `null` and `undefined` collapse into one `nil`.
- `typeof` becomes `type(...)`, and it returns `"nil"` for a missing value.
<!-- /from -->

<!-- from:php -->
## Coming from PHP
- There is no `declare(strict_types=1)` and no parameter types. A function takes whatever you pass it.
- `"10" + 1` is `11` as in PHP, but `"abc" + 1` raises an error instead of giving a warning and a number.
<!-- /from -->

<!-- from:default -->
## Key points
- Types belong to values, and `type(v)` names them.
- `number` has two subtypes, integer and float. `math.type(v)` tells them apart, and `1 == 1.0` is `true`.
- `tonumber(s)` returns `nil` when the text is not a number. `tostring(v)` always works.
- A type error appears only when the bad line runs.
<!-- /from -->

## Common mistake
Assuming `tonumber` fails loudly. `tonumber("abc")` returns `nil`, and the failure shows up later, for example as `attempt to perform arithmetic on a nil value`. Check the result when the input is not trusted.
