# Blocks and Control Flow in Go

Go marks blocks with braces and writes conditions without parentheses. `for` is the only loop: one form is a counter, one is a `while`, a bare `for { }` runs until `break`, and `for ... range` walks a slice, map, string or (Go 1.22+) an integer. `switch` does not fall through, so you do not write `break`. A `switch` with no value is a clean chain of `if`s. There is no ternary operator and no `while` keyword.

```go
package main

import "fmt"

func main() {
	for i := 0; i < 3; i++ {
		fmt.Print(i, " ") // --> 0 1 2
	}
	fmt.Println()
	for i, x := range []string{"a", "b"} {
		fmt.Println(i, x) // --> 0 a   then   1 b
	}
	for i := range 3 { // Go 1.22+
		fmt.Print(i, " ") // --> 0 1 2
	}
	fmt.Println()
	switch d := 3; d {
	case 1, 2:
		fmt.Println("low")
	case 3:
		fmt.Println("three") // --> three
	default:
		fmt.Println("other")
	}
}
```

<!-- from:python -->
## Coming from Python
- `for x in xs` is `for _, x := range xs`. With one variable, `range` gives the index, not the item.
- There is no `while`: write `for cond { }`.
- `switch` replaces `match`, with plain values and no pattern syntax.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- `for ... range` over a slice gives index first, then value.
- `switch` cases never fall through. Use the `fallthrough` keyword if you want that.
- `if` needs a real `bool`: `if count { }` does not compile.
<!-- /from -->

<!-- from:default -->
## Key points
- `for` covers counting, `while`, infinite loops and `range`.
- `break` leaves a loop and `continue` skips to the next round.
- `switch` stops after the matching case; list several values with `case 1, 2:`.
- `if` and `switch` accept a short statement first: `if x := f(); x > 0 { }`.
<!-- /from -->

## Common mistake
Writing `for i, x := range xs` and never using `i`. An unused variable does not compile. Write `for _, x := range xs`.
