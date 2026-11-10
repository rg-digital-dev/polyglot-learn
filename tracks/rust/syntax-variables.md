# Variables and Constants in Rust

`let` creates a variable, and it is immutable by default: you cannot assign to it again. Write `let mut` when the value must change. `const` is for fixed values known at compile time, and it needs a type: `const MAX: i32 = 3;`. You can also **shadow** a name with a new `let`. That creates a new variable that may have a different type, which is not the same as mutating. A variable lives until the end of its `{ }` block. The compiler checks all of this, so a mistake never reaches the tests.

```rust
const MAX: i32 = 3;

fn main() {
    let x = 5;                     // immutable
    let mut count = 0;             // mutable
    count += 1;
    let x = x * 2;                 // shadowing: a new x
    {
        let x = "inner";           // lives only inside this block
        println!("{x}");           // --> inner
    }
    println!("{x} {count} {MAX}"); // --> 10 1 3
    let x = x.to_string();         // shadowing can change the type
    println!("{x}");               // --> 10
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `let` in Rust is closer to `const`, and `let mut` is closer to `let`. The default is the opposite.
- Rust's immutability goes deeper: `let v = vec![1]; v.push(2);` does not compile (E0596). Add `mut` to change the contents.
- Shadowing with `let x = ...` is allowed in the same scope. TypeScript rejects a second `let x`.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- `x = 6` after `let x = 5;` is an error (E0384), not a rebind. Use `let` each time you make a new variable.
- Variables are block scoped. A name made inside an `if` is gone after it.
<!-- /from -->

<!-- from:default -->
## Key points
- `let` is immutable, `let mut` is mutable, `const` needs a type and a value known at compile time.
- Assigning to an immutable variable is error E0384. Calling a method that changes it, like `push`, is E0596.
- Shadowing (`let x = x * 2;`) makes a new variable, so it can change the type.
- A variable can be declared first and assigned once later: `let late; late = 1;`.
<!-- /from -->

## Common mistake
Adding `mut` everywhere to silence the compiler. Add it only to the variables you change, and prefer shadowing for a one-off transformation.
