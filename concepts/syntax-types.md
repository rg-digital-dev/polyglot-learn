# Types and Annotations

A value's **type** says what kind of thing it is: a whole number, a piece of text, a list. Every language has types. They differ in two places: **whether you write them down**, and **who checks them, and when**.

- **Static** languages check types before the program runs. Rust and Go refuse to build if a type doesn't fit, so a whole class of mistakes never reaches the tests.
- **Dynamic** languages check while the program runs, if at all. Lua and Python find out that `"a" + 1` makes no sense only when that line runs.
- **Annotations** sit in between. TypeScript, Python, PHP and QML let you write types down, but each treats them differently. Some check them, some convert the value to fit, and some just ignore them.

| Track | Writing a type | Checked |
|---|---|---|
| TypeScript | `let n: number = 1` | by tsc; Node runs the code without looking |
| Python | `n: int = 1` | never while running (tools like mypy can) |
| Rust | `let n: i64 = 1` | by the compiler, before it runs |
| Go | `var n int = 1` | by the compiler, before it runs |
| PHP | `function f(int $n): int` | by PHP, at each call |
| Lua | none | never: values carry their type |
| QML | `property int n: 1` | by QML, converting the value where it can |

PHP takes types only on parameters, return values and class properties, not on a local variable. QML puts them on properties and on function parameters (`function f(n: int): int`).

## Inference

Writing the type is often optional, even in strict languages: `let n = 1` in Rust, `n := 1` in Go and `let n = 1` in TypeScript all work out the type from the value. The type is still fixed: assigning a string to `n` later doesn't compile in Rust or Go, and tsc reports it in TypeScript.

## The basic types side by side

| Track | Whole number | Decimal | Text | True/false |
|---|---|---|---|---|
| TypeScript | `number` | `number` | `string` | `boolean` |
| Python | `int` | `float` | `str` | `bool` |
| Rust | `i64` | `f64` | `String`, `&str` | `bool` |
| Go | `int` | `float64` | `string` | `bool` |
| PHP | `int` | `float` | `string` | `bool` |
| Lua | integer | float | string | boolean |
| QML | `int`, `real` | `real` | `string` | `bool` |

TypeScript and QML (both JavaScript underneath) have one number type for whole numbers and decimals. Rust has several integer sizes (`i32`, `u8`, `usize`…); the course uses `i64`.

"No value" differs more:

| Track | No value |
|---|---|
| TypeScript, QML | `null` and `undefined` |
| Python | `None` |
| Rust | none: a maybe-missing value is `Option<T>`, `Some(v)` or `None` |
| Go | `nil` for pointers, maps and slices; an `int` is never `nil` |
| PHP | `null` |
| Lua | `nil` |

Rust's way means the compiler makes you handle the missing case before the code builds.

## Converting between types

Rust and Go never convert for you: `i64 + i32` and `int + float64` don't compile until you convert one side. Python refuses `"a" + 1` when it runs. Lua and PHP happily turn `"10" + 1` into `11`. TypeScript and QML turn `"2" + 3` into the string `"23"`, because `+` with a string joins text.

## Common mistake

Trusting an annotation to protect you when nothing checks it. In Python, `def f(n: int)` still accepts a string. In TypeScript, the tests run with Node, which doesn't look at types, so `let s: string = 5` runs fine until you run tsc. The annotation is documentation unless your track's checker actually runs.

Deeper later: Numbers and Division (`int` and float division), Text and Unicode (what a string is).
