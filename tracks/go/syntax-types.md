# Types and Annotations in Go

Go is statically typed: the compiler checks every type before the program runs. You write types on parameters, return values and `var` declarations, and let `:=` infer the rest. Go never converts between types for you. `int` and `float64` are different types, so you convert on purpose with `float64(n)`. A `string` holds bytes, normally UTF-8 text, and `len` counts bytes. Maps, slices and pointers have the zero value `nil`.

```go
package main

import "fmt"

func main() {
	n := 7
	fmt.Println(float64(n) * 2.0)  // --> 14
	fmt.Println(n/2, float64(n)/2) // --> 3 3.5
	s := "Zoë"
	fmt.Println(len(s), len([]rune(s))) // --> 4 3
	var m map[string]int
	var xs []int
	fmt.Println(m == nil, xs == nil) // --> true true
	type Celsius float64
	c := Celsius(21.5)
	fmt.Printf("%T %v\n", c, c) // --> main.Celsius 21.5
}
```

<!-- from:python -->
## Coming from Python
- Types are enforced, not hints. Passing a `string` to an `int` parameter does not compile.
- Two variables of different numeric types never mix: `a + b` with an `int` and a `float64` is a compile error.
- `len("Zoë")` is 4 (bytes). Count characters with `len([]rune(s))`.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- The compiler checks types on every build. There is no separate `tsc` step.
- There is an `int` and a `float64`, not a single `number`.
- There is no `null` or `undefined`. `nil` exists only for pointers, maps, slices, channels, functions and interfaces.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- There is no `Option<T>`. A missing value is `nil` for reference-like types, or the zero value for the rest.
- `int` is 32 or 64 bits depending on the platform (64 here).
- Integers wrap silently on overflow: a `uint8` holding 255, plus 1, becomes 0.
<!-- /from -->

<!-- from:default -->
## Key points
- Types are checked at compile time; a mismatch stops the build.
- No implicit conversions: write `float64(n)` or `int(f)`.
- `:=` infers: `1` is `int`, `1.5` is `float64`, `'a'` is a `rune` (`int32`).
- Every type has a zero value, so a declared variable is always usable.
<!-- /from -->

## Common mistake
Dividing two integers and expecting a decimal. `7 / 2` is `3`. Convert one side first: `float64(7) / 2`.
