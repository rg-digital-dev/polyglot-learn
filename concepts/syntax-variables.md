# Variables and Constants

A **variable** is a name for a value. Three questions tell the languages apart:

1. **How do you declare one?** With a keyword (`let`, `var`, `local`), with a sigil (`$x` in PHP), or just by assigning (`x = 1` in Python).
2. **Can you point the name at a new value later?** That's *reassigning*. Some languages make names fixed by default and ask you to opt in (`let mut` in Rust); most do the opposite (`const` to opt out).
3. **Who stops a mistake, and when?** A compiler before anything runs, the runtime when the line runs, or nobody.

| Track | Can change | Can't be reassigned |
|---|---|---|
| TypeScript | `let x = 1` | `const x = 1` |
| Python | `x = 1` | none (`Final` is only a hint) |
| Rust | `let mut x = 1` | `let x = 1`, `const X: i32 = 1` |
| Go | `x := 1`, `var x = 1` | `const X = 1` |
| PHP | `$x = 1` | `const X = 1;` |
| Lua | `local x = 1` | `local x <const> = 1` |
| QML | `let x`, `property int x: 1` | `const x`, `readonly property` |

| Track | Reassigning a fixed name is caught… |
|---|---|
| TypeScript | by tsc, or with a TypeError when the line runs under Node |
| Python | never |
| Rust, Go | by the compiler, before anything runs |
| PHP | when PHP reads the file (and `const` only works at top level or in a class) |
| Lua | when Lua loads the file |
| QML | with a TypeError when the line runs |

## "Can't be reassigned" is about the name, not the value

`const` stops the *name* from pointing somewhere else. It doesn't freeze what the name points to:

```text
const xs = [1, 2]
xs.push(3)      // fine: same list, one more item
xs = [9]        // error: xs must keep pointing at the same list
```

That holds for TypeScript, QML, Lua's `<const>` and Go (which only allows `const` for numbers, strings and booleans anyway). Rust is the exception: a `let` binding without `mut` can't be changed through at all, so `v.push(3)` on it doesn't compile.

## Scope

A variable exists from where it's declared to the end of its **scope**. In most of these languages the scope is the nearest `{ … }` block. Python, PHP and the old `var` (in TypeScript and QML) are different: a variable made inside an `if` is still there after it, until the function ends. Lua has the opposite trap: leave out `local` and you create a global, visible everywhere.

## Common mistake

Reaching for `var` (TypeScript, QML) or forgetting `local` (Lua) out of habit. Both widen the variable's scope past what you meant. Declare with the narrowest form your language has, and only make a name changeable when you actually change it.

Deeper later: Values and References (`make-grid`, `add-bonus`), on what assigning a list really copies.
