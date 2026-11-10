# Lists, Maps and Tuples

Most exercises take or return a group of values. Three shapes cover nearly all of them:

- A **list** is an ordered row of values, reached by position: the first, the second…
- A **map** (dictionary, hash, object) pairs **keys** with values, reached by key: the score for `"alice"`.
- A **tuple** is a small, fixed group of values that belong together, such as a name and a score.

## Lists

| Track | Literal | First item | Length | Add to the end |
|---|---|---|---|---|
| TypeScript, QML | `[1, 2, 3]` | `xs[0]` | `xs.length` | `xs.push(4)` |
| Python | `[1, 2, 3]` | `xs[0]` | `len(xs)` | `xs.append(4)` |
| Rust | `vec![1, 2, 3]` | `xs[0]` | `xs.len()` | `xs.push(4)` |
| Go | `[]int{1, 2, 3}` | `xs[0]` | `len(xs)` | `xs = append(xs, 4)` |
| PHP | `[1, 2, 3]` | `$xs[0]` | `count($xs)` | `$xs[] = 4;` |
| Lua | `{1, 2, 3}` | `xs[1]` | `#xs` | `xs[#xs + 1] = 4` |

Lua counts from **1**: `xs[1]` is the first item and `xs[0]` is `nil`. Every other track counts from 0. In Go, `append` returns the longer list, so you assign it back.

## Maps

| Track | Literal | Read a key |
|---|---|---|
| TypeScript, QML | `{ alice: 3 }` | `m.alice` or `m["alice"]` |
| Python | `{"alice": 3}` | `m["alice"]` |
| Rust | `HashMap::from([("alice", 3)])` | `m.get("alice")` |
| Go | `map[string]int{"alice": 3}` | `m["alice"]` |
| PHP | `["alice" => 3]` | `$m["alice"]` |
| Lua | `{alice = 3}` | `m.alice` or `m["alice"]` |

TypeScript and QML also have `new Map()`, read with `m.get("alice")`, which is the better choice when the keys come from data.

| Track | Reading a missing key gives… |
|---|---|
| TypeScript, QML | `undefined` |
| Python | a `KeyError` (`m.get(k)` gives `None` instead) |
| Rust | `None`: `get` returns an `Option` |
| Go | the zero value, such as `0`; `v, ok := m[k]` tells you which |
| PHP | `null`, and a warning |
| Lua | `nil` |

PHP and Lua use the same structure for both shapes: a PHP array and a Lua table are lists and maps at once.

## Tuples

Python and Rust have real tuples: `("alice", 3)`. In TypeScript the type `[string, number]` describes an array used as a tuple. Go, PHP, Lua and QML have no tuple; use a small struct, object or table, or (Go and Lua) return several values.

## Common mistake

Off-by-one between tracks. A loop from 1 to `#xs` is right in Lua and skips the first item anywhere else; a loop from 0 to `len - 1` is right everywhere else and misses the last item in Lua.

Deeper later: Maps (`word-count`), Sorting (`top-scores`), Values and References (`make-grid`).
