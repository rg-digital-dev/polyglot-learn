# Operators and Strings

An **expression** is anything that produces a value: `1 + 2`, `name == "Bob"`, `greet("Alice")`. Operators look the same across languages until they don't. Three families cause most of the surprises: equality, joining text, and the logical operators.

## Equality

`==` asks "are these equal?", but each language has its own idea of equal.

| Track | Equal | Not equal | Is `1` equal to `"1"`? |
|---|---|---|---|
| TypeScript, PHP, QML | `===` | `!==` | no (`==` says yes) |
| Python | `==` | `!=` | no |
| Rust, Go | `==` | `!=` | won't compile |
| Lua | `==` | `~=` | no |

TypeScript, PHP and QML also have a two-character `==`, which converts both sides to the same type first. Use the three-character forms unless you really mean "equal after converting". Python's `is` asks a different question: "the very same object?".

## Joining text

| Track | Join | Put values inside text |
|---|---|---|
| TypeScript, QML | `"Hi, " + name` | `` `Hi, ${name}!` `` |
| Python | `"Hi, " + name` | `f"Hi, {name}!"` |
| Rust | `s + "!"` | `format!("Hi, {name}!")` |
| Go | `"Hi, " + name` | `fmt.Sprintf("Hi, %s!", name)` |
| PHP | `"Hi, " . $name` | `"Hi, {$name}!"` |
| Lua | `"Hi, " .. name` | `string.format("Hi, %s!", name)` |

Putting values inside a template is usually clearer than chaining joins, and it's what the `hello` hints suggest. PHP only fills in `{$name}` inside double quotes; single quotes keep the text as written.

## Logical operators

`and`, `or` and `not` combine true/false values. Every language stops early: in `a or b`, `b` isn't evaluated when `a` is already true.

| Track | And | Or | Not |
|---|---|---|---|
| TypeScript, Rust, Go, QML | `&&` | `\|\|` | `!` |
| Python, Lua | `and` | `or` | `not` |
| PHP | `&&` | `\|\|` | `!` |

PHP also has `and` and `or`, but they bind more loosely than `=`, so `$ok = true and false;` leaves `$ok` true. Stick to `&&` and `||` there.

In TypeScript, Python, Lua and QML, `a or b` gives back one of the two values, not a strict boolean, which makes "this, or a default" a one-liner: `name || "world"`, `name or "world"`. Which values count as "false" differs per language; the Conditionals lesson covers it.

## Calling functions and methods

Some operations are **methods**, written after the value with a dot. Others are plain **functions** that take the value as an argument. Trimming spaces off a string, which `hello` needs, shows the split:

| Track | Trim a string |
|---|---|
| TypeScript, QML | `name.trim()` |
| Python | `name.strip()` |
| Rust | `name.trim()` |
| Go | `strings.TrimSpace(name)` |
| PHP | `trim($name)` |
| Lua | `name:match("^%s*(.-)%s*$")` (no built-in trim) |

Lua's `:` is its method call: `s:upper()` is short for `string.upper(s)`.

## Common mistake

Using `+` to join text where the language means numbers. `"Score: " + 5` is an error in Python, Rust, Go, PHP and Lua (the last two try to turn the text into a number and fail). Only TypeScript and QML turn it into `"Score: 5"`. Use the join operator from the table, or put the value inside a template.

Deeper later: Conditionals (`leap-year`), Numbers and Division (`/` and `%`).
