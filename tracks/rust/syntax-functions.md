# Functions in Rust

A function is `fn name(param: Type, ...) -> ReturnType { body }`. Every parameter needs a type, and so does the result (leave out `->` when nothing is returned, which means the unit value `()`). The last expression of the body, with no `;`, is the result. `return` is for leaving early. To return several values, return a tuple and unpack it: `let (q, r) = divmod(7, 2);`. A function without a name is a closure: `|x| x * 2`. There are no default arguments and no overloading, and a call with the wrong number of arguments does not compile. For text, the stubs take `&str` (borrowed text) and return `String` (owned text); the later References lessons explain why.

```rust
fn add(a: i64, b: i64) -> i64 {
    a + b // no ;, so this is the result
}

fn divmod(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

fn main() {
    let (q, r) = divmod(7, 2);
    let double = |x: i64| x * 2;
    println!("{} {q} {r} {}", add(2, 3), double(4)); // --> 5 3 1 8
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Parameter and return types are required. Only closures can leave them out.
- The last expression is returned. `a + b;` with a `;` returns nothing and fails with `expected i64, found ()`.
- There are no default or optional parameters. Use `Option<T>` when a value may be absent.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- There are no default values, keyword arguments or `*args`. Pass exactly the declared parameters, in order.
- Several results come back as a tuple, as in Python, and you unpack with `let (q, r) = ...`.
<!-- /from -->

<!-- from:default -->
## Key points
- Read the signature first: parameters in the parentheses, result after `->`.
- The final expression without `;` is the return value. `return` is for early exits.
- Tests check what the function returns, not what it prints.
<!-- /from -->

## Common mistake
Putting a `;` on the last line. The value is discarded and the compiler reports `mismatched types`, with a hint to remove the semicolon.
