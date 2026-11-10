# Functions in Go

A function is `func name(params) result { ... }`. Parameter names come before their types, and `a, b int` shares one type between two parameters. You must write `return`, because there is no implicit last-expression value. A function can return several values, which is how Go reports errors: `(int, error)`. Functions have no default parameter values. A function without a name, `func(n int) int { ... }`, can be stored in a variable.

```go
package main

import (
	"errors"
	"fmt"
)

func Add(a, b int) int { return a + b }

func DivMod(a, b int) (int, int) { return a / b, a % b }

func Safe(a, b int) (int, error) {
	if b == 0 {
		return 0, errors.New("divide by zero")
	}
	return a / b, nil
}

func main() {
	fmt.Println(Add(2, 3)) // --> 5
	q, r := DivMod(7, 2)
	fmt.Println(q, r) // --> 3 1
	_, err := Safe(1, 0)
	fmt.Println(err) // --> divide by zero
	double := func(n int) int { return n * 2 }
	fmt.Println(double(4)) // --> 8
}
```

<!-- from:python -->
## Coming from Python
- Parameter and result types are required, and they are checked.
- There are no default or keyword arguments. Pass every argument, in order.
- Returning two values is a real feature, not a tuple. The caller takes both: `q, r := DivMod(7, 2)`. Use `_` to ignore one.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Types follow the name with no colon: `a, b int`.
- The return type goes after the parameter list. Several results go in parentheses.
- No default values, no optional parameters and no arrow functions. Use `func(...) ... { }`.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- The last expression is not returned. Write `return`, or you get "missing return".
- A bare `a + b` line is a compile error ("is not used"), not a return value.
<!-- /from -->

<!-- from:default -->
## Key points
- `func Name(a, b int) int`: name, parameters with types, then the result type.
- A capital first letter exports the function.
- Multiple return values are normal; `(value, error)` is the common pair.
- A function with a result type must `return` on every path.
<!-- /from -->

## Common mistake
Printing the result instead of returning it. The tests read what the function returns, not what it writes to the terminal.
