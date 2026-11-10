# A File at a Glance

This primer is reading only: no exercises and no tests. Each page explains one piece of syntax and shows how all seven tracks write it, with your track's note under it. Skip whatever you already know. Nothing here is required before `hello`.

Every exercise starts as one small file with one function in it. The function body is a placeholder that fails on purpose ("TODO: implement …"), and your job is to replace that body. The tests live in a separate file that `learn` writes for you; you don't edit them.

Before the first exercise, it helps to know what the rest of that file is:

- **Comments.** Text the language ignores. The stubs use them to describe the function.
- **Statements.** One instruction, usually one line. Some languages end each one with `;`, some with the end of the line.
- **The "make it visible" marker.** The tests live in another file, so your function has to be visible from outside its own file. Each language marks that differently.
- **Printing.** You won't need it for the tests, but a quick print is the easiest way to see what a value really is.

| Track | Comment | Statements end with | Print |
|---|---|---|---|
| TypeScript | `// …` | `;` (optional) | `console.log(x)` |
| Python | `# …` | end of line | `print(x)` |
| Rust | `// …` | `;` | `println!("{x}")` |
| Go | `// …` | end of line | `fmt.Println(x)` |
| PHP | `// …` or `# …` | `;` | `echo $x;` |
| Lua | `-- …` | end of line | `print(x)` |
| QML | `// …` | end of line | `console.log(x)` |

| Track | Your function is visible to the tests because… |
|---|---|
| TypeScript | it's marked `export function` |
| Python, PHP | every top-level function is |
| Rust | it's marked `pub fn` |
| Go | its name starts with a capital: `Greet` |
| Lua | it's stored in the module table `M` and the file ends with `return M` |
| QML | it's a `function` of the component in the file |

## Names

The exercises are defined once, in a neutral format, and each track turns the names into its own style. The `order-total` exercise's function, for example, is:

| Track | Function and one parameter | Style |
|---|---|---|
| Python, Rust, Lua | `order_total(…, discount_percent)` | snake_case |
| TypeScript, PHP, QML | `orderTotal(…, discountPercent)` | camelCase |
| Go | `OrderTotal(…, discountPercent)` | capital first: exported |

Use the name the stub gives you.

## Types in the signature

The stub's first line already tells you what goes in and what comes out: `greet(name: string): string` in TypeScript, `def greet(name: str) -> str` in Python, `fn greet(name: &str) -> String` in Rust. The *Types* page explains where each language puts them and who checks them. For now: the part after the name is the input, the part after `:` or `->` at the end is what you return.

## Common mistake

Changing the function's name or its parameters. The tests call it exactly as the stub declares it. Change the body, not the first line.

Deeper later: the `hello` lesson (functions) and the Modules lesson (`run-all`).
