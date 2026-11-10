# A File at a Glance in Rust

A Rust exercise is a library crate. The function you edit lives in `src/lib.rs`, and the tests live in `tests/spec.rs`, which calls it from outside. That is why the stub says `pub fn`: without `pub`, the function is private to its own file and the tests cannot see it. Comments start with `//`, and `///` is a doc comment that describes the item under it. Statements end with `;`. The last line of a function body has no `;`, because that expression is the return value. The placeholder body is `todo!(...)`, a macro that panics with "not yet implemented" until you replace it. Names ending in `!` are macros, such as `println!` and `format!`.

```rust
/// Return a greeting for name.
pub fn greet(name: &str) -> String {
    let name = name.trim(); // a statement ends with ;
    format!("Hello, {name}!") // no ;, so this is the result
}

fn main() {
    println!("{}", greet("  Ann ")); // --> Hello, Ann!
    let n = 3;
    println!("{n} {n:?} {:?}", "hi"); // --> 3 3 "hi"
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `pub fn` is the counterpart of `export function`. Everything is private by default.
- `println!("{x}")` puts a variable in the text, like `${x}` in a template literal. `{x:?}` prints the debug form, which shows quotes on strings.
- The `;` matters: `a + b` returns the value and `a + b;` throws it away.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Indentation is only for readers. Blocks use `{ }` and statements use `;`.
- `println!` is a macro, which is why it has a `!`.
<!-- /from -->

<!-- from:default -->
## Key points
- `pub` makes a function visible to the tests. The stub already has it.
- Change the body of the stub, never its first line: the tests call it exactly as declared.
- `todo!(...)` fails on purpose. Replace it with real code.
- `println!("{x:?}")` is the quickest way to see what a value really is.
<!-- /from -->

## Common mistake
Adding a `;` after the last expression. The function then returns `()` and the compiler reports `expected String, found ()`. Remove the `;`.
