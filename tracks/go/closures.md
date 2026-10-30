# Closures in Go

Functions are values in Go: you can store them in variables, pass them around and return them. A function literal, `func(x int) int { ... }`, can use variables declared in the surrounding function, and it keeps them alive for as long as the literal itself exists. The literal shares those variables rather than copying them, so changes it makes are still there on the next call. A function that returns another function writes the type out in full, for example `func() int`.

```go
func MakeGreeter(greeting string) func(string) string {
	return func(name string) string {
		return greeting + ", " + name + "!"
	}
}

func RunningTotal() func(int) int {
	total := 0
	return func(amount int) int {
		total += amount
		return total
	}
}

// till := RunningTotal()
// till(5) == 5, then till(3) == 8
```

<!-- from:typescript -->
## Coming from TypeScript
- This works like a JS closure: the returned function captures `total` by reference and keeps it alive.
- The function type is written `func(int) int` instead of `(amount: number) => number`.
- There are no arrow functions; a literal always uses the `func` keyword and an explicit `return`.
- Each call to `RunningTotal()` creates a fresh `total`, so two totals never share state.
<!-- /from -->

<!-- from:default -->
## Key points
- Declare the state inside the outer function so each returned function gets its own copy.
- The closure may freely modify captured variables; no extra keyword is needed.
- Since Go 1.22 every loop iteration has its own loop variable, so closures created in a loop no longer share it.
<!-- /from -->

## Common mistake
Keeping the state at package level:

```go
var total int                    // one variable for the whole package
func RunningTotal() func(int) int {
	return func(amount int) int { total += amount; return total }
}
// two totals now add into the same variable; move `total := 0` inside RunningTotal
```

## In this exercise
Implement `func MakeCounter(start int, step int) func() int`. Each call returns the current value and then advances it by `step`, so the first call returns `start`, and every counter keeps its own state.
