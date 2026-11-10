# A File at a Glance in Go

A Go file starts with `package`, then its imports, then declarations. The exercise stub is one `.go` file with one function whose body is `panic("TODO: implement Greet")`. You replace the panic with real code. Comments start with `//`. A statement ends at the end of the line, so you never write `;`. The tests live in another file in the same folder, so they can call your function only if its name starts with a capital letter. That is Go's whole visibility rule: `Greet` is exported, `greet` is private to the package.

```go
package main

import "fmt"

// Greet is visible outside the package because of the capital G.
func Greet(name string) string {
	return "Hello, " + name + "!"
}

func greetLoud(name string) string { // lower case: private to the package
	return Greet(name) + "!!"
}

func main() {
	fmt.Println(Greet("Alice"))   // --> Hello, Alice!
	fmt.Println(greetLoud("Bob")) // --> Hello, Bob!!!
}
```

<!-- from:python -->
## Coming from Python
- Blocks use braces, not indentation. Indentation is only for readers, and `gofmt` sets it for you.
- Every file begins with `package`, and every other package you use needs an `import`.
- Visibility is the first letter of the name, not a leading underscore.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- There is no `export` keyword. A capital first letter exports the name.
- There is no `;` and no `console.log`. Print with `fmt.Println`.
- The opening brace must stay on the same line as `func`.
<!-- /from -->

<!-- from:default -->
## Key points
- `package` first, then `import`, then declarations.
- A capital first letter means exported (visible to the tests); lower case means private.
- `fmt.Println(x)` prints a value and adds a newline.
- Change the body of the stub, not its first line.
<!-- /from -->

## Common mistake
Renaming `Greet` to `greet`. The code still compiles, but the tests can no longer see the function and fail to build.
