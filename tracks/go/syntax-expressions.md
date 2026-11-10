# Operators and Strings in Go

Go uses `==` and `!=` for equality, and both sides must have the same type, so `1 == "1"` does not compile. Logical operators are `&&`, `||` and `!`, and they short-circuit. Join strings with `+`, or build one with `fmt.Sprintf` and verbs such as `%s`, `%d`, `%v` and `%q`. Integer division truncates toward zero, and `%` takes the sign of the left operand. Text helpers live in the `strings` package.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	name := "Ann"
	fmt.Println("Hi, " + name + "!")                          // --> Hi, Ann!
	fmt.Println(fmt.Sprintf("Hi, %s! You are %d.", name, 30)) // --> Hi, Ann! You are 30.
	fmt.Println(7/2, -7/2, 7%3, -7%3, 7.0/2)                  // --> 3 -3 1 -1 3.5
	fmt.Println(strings.TrimSpace("  hi  "))                  // --> hi
	i := 3
	i++
	i += 2
	fmt.Println(i, "abc" < "abd") // --> 6 true
}
```

<!-- from:python -->
## Coming from Python
- There are no f-strings. Use `fmt.Sprintf("%s", x)`.
- `and`, `or` and `not` are `&&`, `||` and `!`. They give a `bool`, never an operand, so there is no `x or "default"`.
- `7 / 2` is `3`, not `3.5`, and there is no `//` operator.
- `i++` is a statement, not an expression, and there is no `++i`.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- `+` never joins a string and a number: `"Score: " + 5` does not compile. Use `fmt.Sprintf("Score: %d", 5)`.
- There is no `===` and no loose `==`. One `==`, strict, same types only.
- Strings use double quotes. Single quotes make one character (a `rune`).
<!-- /from -->

<!-- from:php -->
## Coming from PHP
- Join with `+`, not `.`.
- No type juggling: `"10" + 1` does not compile.
- Strings do not interpolate variables. Use `fmt.Sprintf`.
<!-- /from -->

<!-- from:default -->
## Key points
- `==` needs matching types on both sides.
- `+` joins strings; `fmt.Sprintf` formats them. `%v` prints any value.
- `/` between integers truncates; `%` keeps the left operand's sign.
- `strings.TrimSpace(s)` trims; a Go string has no methods.
<!-- /from -->

## Common mistake
Mixing a string and a number with `+`. Convert first with `strconv.Itoa(5)`, or format with `Sprintf`.
