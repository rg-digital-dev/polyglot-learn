# Blocks and Control Flow

**Control flow** decides which lines run, and how often. It's built from **blocks**: a group of statements that belong together, such as the body of an `if` or a loop. The languages mark a block in three ways:

- **Braces** `{ … }`: TypeScript, Rust, Go, PHP and QML. Indentation is only for readers.
- **Indentation**: Python. A `:` opens the block, and the indented lines are its body. The indentation *is* the syntax.
- **Keywords**: Lua. `then … end` and `do … end`.

## Choosing: `if`

| Track | If / else if / else |
|---|---|
| TypeScript, PHP, QML | `if (a) { … } else if (b) { … } else { … }` |
| Python | `if a:` / `elif b:` / `else:` |
| Rust | `if a { … } else if b { … } else { … }` |
| Go | `if a { … } else if b { … } else { … }` |
| Lua | `if a then … elseif b then … else … end` |

PHP also accepts `elseif` as one word. Rust and Go leave out the parentheses around the condition but always need the braces. Rust's `if` is also an expression that gives a value: `let size = if n > 5 { "big" } else { "small" };`. The others use a shorter form for that: `n > 5 ? "big" : "small"` (TypeScript, PHP, QML), `"big" if n > 5 else "small"` (Python), `n > 5 and "big" or "small"` (Lua). Go has none; write a full `if`.

## Repeating: loops

| Track | Each item of a list | Counting from 0 to 9 |
|---|---|---|
| TypeScript, QML | `for (const x of xs)` | `for (let i = 0; i < 10; i++)` |
| Python | `for x in xs:` | `for i in range(10):` |
| Rust | `for x in &xs` | `for i in 0..10` |
| Go | `for _, x := range xs` | `for i := 0; i < 10; i++` |
| PHP | `foreach ($xs as $x)` | `for ($i = 0; $i < 10; $i++)` |
| Lua | `for _, x in ipairs(xs) do` | `for i = 0, 9 do` |

Every language also has a "while this is true" loop: `while` everywhere, plus Rust's `loop` (until `break`), Lua's `repeat … until` and Go's bare `for { … }`. Go has no `while` keyword: `for` covers all of them. `break` leaves a loop and `continue` skips to the next round in every track except Lua, which has `break` but no `continue`.

Watch the ranges: Rust's `0..10` and Python's `range(10)` stop *before* 10, while Lua's `for i = 0, 9` includes 9.

## Matching one value: `switch` and `match`

| Track | Form | Falls through? |
|---|---|---|
| TypeScript, QML | `switch (x) { case 1: … break; }` | yes, until `break` |
| Python | `match x:` / `case 1:` / `case _:` | no |
| Rust | `match x { 1 => …, _ => … }` | no (must cover all) |
| Go | `switch x { case 1: … }` | no |
| PHP | `match ($x) { 1 => …, default => … }` | no (unmatched throws) |
| Lua | none: use `if … elseif` | |

"Falls through" means that after the matching case runs, the next case's code runs too. The "catch everything else" case is `default` (TypeScript, QML, Go, PHP) or `_` (Python, Rust). Rust refuses to compile a `match` that misses a possible value, and PHP's `match` throws an `UnhandledMatchError` when nothing matches.

PHP also has a `switch`, which falls through like TypeScript's and compares loosely (`"5"` matches `case 5`). Its newer `match` compares strictly, which is usually what you want.

## Common mistake

A missing `break` in a TypeScript, QML or PHP `switch`. The case runs, then the next case's code runs too. Put a `break` (or `return`) at the end of every case.

Deeper later: Conditionals (`leap-year`), Loops (`sum-of-multiples`), Iterators (`top-words`).
