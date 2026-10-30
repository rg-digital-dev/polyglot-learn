# Loops in Go

Go has a single loop keyword, `for`, in a few forms: the three-part `for i := 0; i < n; i++`, a condition-only `for cond` (a "while" loop), a bare `for` that runs until `break`, and `for ... range` to walk a slice, map, string or channel. Since Go 1.22 you can also range over an integer: `for i := range 5` gives 0 to 4. `continue` skips to the next pass and `break` leaves the loop.

```go
func SumOfEvenSquares(limit int) int {
	total := 0
	for n := 1; n < limit; n++ {
		if n%2 != 0 {
			continue
		}
		total += n * n
	}
	return total
}

func ContainsNegative(values []int) bool {
	for _, v := range values {
		if v < 0 {
			return true
		}
	}
	return false
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `for` covers `for`, `while` and `do`-style loops; there is no `while` keyword.
- `for i, v := range values` gives index and value, like `entries()`; use `_` for the one you skip.
- No parentheses around the loop header, and braces are always required.
- There are no `map`/`filter`/`some` methods on slices; a plain loop is the idiom.
- Integer `n % 0` panics at runtime instead of producing `NaN`.
<!-- /from -->

<!-- from:default -->
## Key points
- `range` over a slice yields `index, value`; write `_` to ignore the index.
- Use `i < limit` for an exclusive upper bound.
- Return from inside the loop as soon as the answer is known.
<!-- /from -->

## Common mistake
Ranging with a single variable gives the index, not the element:

```go
for f := range factors { total += f }     // adds 0, 1, 2, ... (indexes)
for _, f := range factors { total += f }  // adds the values
```

## In this exercise
Implement `func SumOfMultiples(factors []int, limit int) int`. Skip a factor of 0 before using `%`, or the test run will panic with "integer divide by zero".
