# Operators and Strings in Lua

Lua spells a few operators with words or unusual symbols. Equality is `==` and inequality is `~=`, and neither converts types: `1 == "1"` is `false`. Text joins with `..`. The logical operators are `and`, `or` and `not`; `and` and `or` stop early and return one of their operands, so `name or "world"` is a default. `^` is power (always a float), `//` is floor division and `%` is the remainder, with the sign of the divisor. Strings are immutable bytes with no interpolation, so you build text with `..` or `string.format`. Lua has no built-in trim, so `hello` uses a pattern. Methods use `:`.

```lua
print(1 == "1", 1 ~= 2, "a" < "b")             --> false   true   true
print("Hi, " .. "Bob", 1 .. 2)                 --> Hi, Bob   12
print(string.format("%s is %d, %.2f", "x", 7, 3.14159))
--> x is 7, 3.14
print(nil or "world", 0 and "yes", nil and 1)  --> world   yes   nil
print(2 ^ 3, 7 % 3, -7 % 3, -7 // 2)           --> 8.0   1   2   -4
print(#"hello", ("hi"):upper())                --> 5   HI
print(("  hi  "):match("^%s*(.-)%s*$"))        --> hi
print(pcall(function() return 1 < "2" end))
--> false   ...: attempt to compare number with string
```

<!-- from:python -->
## Coming from Python
- `+` never joins text. `"a" + "b"` is an error; write `"a" .. "b"`.
- There are no f-strings. Use `string.format("%s", x)` or `..`.
- Comparisons do not chain. `1 < 2 < 3` fails because `true < 3` is an error.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Write `~=`, not `!=` or `!==`. `==` already behaves like `===`.
- `&&`, `||` and `!` become `and`, `or` and `not`.
- `"Score: " + 5` is an error. Use `"Score: " .. 5`, which gives `"Score: 5"`.
- `0` and `""` are truthy, so `x or "default"` replaces only `nil` and `false`.
<!-- /from -->

<!-- from:default -->
## Key points
- `==` and `~=` compare without converting types.
- `..` joins, `string.format` fills placeholders, and `s:method()` calls `string.method(s)`.
- `and` and `or` return an operand, and only `nil` and `false` count as false.
- `#s` counts bytes, not characters.
<!-- /from -->

## Common mistake
Writing `a and b or c` as a ternary when `b` can be `false` or `nil`. The result is then `c` even when `a` is true. Use a full `if` in that case.
