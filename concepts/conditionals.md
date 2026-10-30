# Conditionals

Conditionals choose between paths based on a boolean. Every language has `if` and `else`. The differences are in what counts as a boolean, and in whether `if` is a statement or an expression that produces a value.

- **Truthiness.** JavaScript, PHP and Lua let non-booleans stand in for true and false, and each has its own rules: `0` is falsy in JS and PHP but truthy in Lua. Rust and Go only accept a real `bool`.
- **`if` as a value.** In Rust, `if` is an expression (`let x = if a { 1 } else { 2 };`). Most other languages use a ternary (`a ? 1 : 2`) or a plain statement.
- **Operators.** `&&` and `||` short-circuit everywhere. Lua spells them `and` and `or`, and its "not equal" is `~=`.

The leap-year rule is a classic test of nested conditions:

```text
divisible by 4      -> leap year
  ...unless by 100  -> not a leap year
  ...unless by 400  -> leap year again
```

It can be written as nested `if`s, as an early-return chain, or as one boolean expression: `y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)`. All three are fine. Choose the one you'd want to read six months from now.

## Common mistake

Checking the rules in the wrong order. If you test "divisible by 100 → false" before "divisible by 400 → true", the year 2000 comes out wrong. Test the most specific rule first, or combine them as above.

## Go deeper

- The rule itself: [Leap year (Wikipedia)](https://en.wikipedia.org/wiki/Leap_year)
- Exercism's language-neutral spec: [leap](https://github.com/exercism/problem-specifications/tree/main/exercises/leap)
