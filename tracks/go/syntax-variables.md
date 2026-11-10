# Variables and Constants in Go

Go has two ways to declare a variable. `x := 1` declares and infers the type, but only inside a function. `var x int` works everywhere and gives the variable its zero value (`0`, `""`, `false`, `nil`), so a variable is never uninitialised. `const` names a fixed number, string or boolean, and assigning to one is a compile error. An unused local variable is also a compile error. The compiler checks all of this before anything runs.

```go
package main

import "fmt"

const Max = 3

func main() {
	count := 1
	count = 2 // plain = assigns an existing variable
	var name string
	var n int
	fmt.Printf("%q %d\n", name, n) // --> "" 0
	var rate float64 = 1.5
	fmt.Println(Max*rate, count) // --> 4.5 2
	if x := 5; x > 3 {
		fmt.Println("x is", x) // --> x is 5
	}
	a, b := 1, 2
	a, b = b, a
	fmt.Println(a, b) // --> 2 1
}
```

<!-- from:python -->
## Coming from Python
- Declare a variable once with `:=` or `var`. After that, `=` assigns.
- A variable keeps one type for life. `x := 1` then `x = "a"` does not compile.
- An unused local variable is an error, not a warning. Use `_` to discard a value.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- `const` only works for numbers, strings and booleans. A slice or map cannot be `const`.
- There is no `let`. Use `:=` for the short form.
- `:=` at the top level of a file is a syntax error. Use `var` there.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- Variables are mutable by default. There is no `mut`, and `const` is the only read-only form (simple values only).
- `:=` with no new name on its left side does not compile, and the compiler will not shadow within one scope. A new scope (a `{ }` block) can reuse a name.
<!-- /from -->

<!-- from:default -->
## Key points
- `:=` declares and infers; `var` declares, optionally with a type and a zero value.
- `:=` works only inside functions.
- Constants cannot be reassigned, and unused locals do not compile.
- Scope is the nearest `{ ... }`. In `if x := ...; cond`, `x` lives only in that `if`.
<!-- /from -->

## Common mistake
Writing `:=` when the variable already exists. `count := 2` after `count := 1` fails with "no new variables on left side of :=". Use `count = 2`.
