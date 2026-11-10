# Functions

A **function** is a named, reusable piece of code: it takes **parameters**, does something, and **returns** a value. Every exercise in this course is one function, so its first line, the **signature**, is the first thing you read.

| Track | A function that adds two whole numbers |
|---|---|
| TypeScript | `function add(a: number, b: number): number { return a + b; }` |
| Python | `def add(a: int, b: int) -> int:` + indented `return a + b` |
| Rust | `fn add(a: i64, b: i64) -> i64 { a + b }` |
| Go | `func add(a, b int) int { return a + b }` |
| PHP | `function add(int $a, int $b): int { return $a + $b; }` |
| Lua | `local function add(a, b) return a + b end` |
| QML | `function add(a: int, b: int): int { return a + b }` |

Read them the same way: the name, the parameters in parentheses with their types, then the return type after `:`, `->` or the closing parenthesis (Go).

- **Go** writes the type after the name without a colon, and `a, b int` shares one type between both parameters.
- **Rust** returns the last expression of the body if it has no `;`. `a + b` with no semicolon is the return value; `return a + b;` works too.
- **Lua** has no types in the signature at all. The course's stubs describe them in `--- @param` comments instead.

## Defaults, several results, functions as values

| Track | Default value | Return two values | A function without a name |
|---|---|---|---|
| TypeScript | `b = 1` | an array `[q, r]` | `(a, b) => a + b` |
| Python | `b=1` | a tuple `return q, r` | `lambda a, b: a + b` |
| Rust | none | a tuple `(q, r)` | `\|a, b\| a + b` |
| Go | none | `func f() (int, int)` | `func(a, b int) int { … }` |
| PHP | `$b = 1` | an array `[$q, $r]` | `fn($a, $b) => $a + $b` |
| Lua | `b = b or 1` in the body | `return q, r` | `function(a, b) … end` |
| QML | `b = 1` | an array `[q, r]` | `(a, b) => a + b` |

Go and Lua return several values for real. The others return one value that holds both (a tuple or an array), which the caller unpacks: `const [q, r] = f()`, `q, r = f()`, `let (q, r) = f();`.

## Common mistake

Printing the answer instead of returning it. The tests look at what the function *returns*. A `print` shows the right text in your terminal and still fails the test.

Deeper later: Functions (`hello`), Closures (`make-counter`).
