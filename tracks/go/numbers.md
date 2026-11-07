# Numbers in Go

`int` is 64 bits on every platform the course runs on. Integer `/` truncates toward zero and `%` takes the dividend's sign, and there is no floor-division operator: you adjust a negative remainder yourself. Overflow wraps around silently at run time. Only *constant* expressions are checked: `math.MaxInt64 + 1` written out is a compile error, but the same sum on variables wraps. `math.Round` rounds halves away from zero and `math.RoundToEven` to even, both on `float64`.

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	a, b := -7, 2
	fmt.Println(a/b, a%b)                              // -3 -1
	fmt.Println(math.Round(-2.5), math.RoundToEven(2.5)) // -3 2
	big := 4294967296
	fmt.Println(big * big)                             // 0
	max := math.MaxInt64
	fmt.Println(max + 1 == math.MinInt64)              // true
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Integer `/` is `Math.trunc(a / b)`, and `%` is the same remainder as in JavaScript, so `-1 % 5` is `-1` here too (but never `-0`).
- Numbers don't lose precision past 2^53; they wrap past 2^63, without an error.
- `math.Round(-2.5)` is `-3`; `Math.round(-2.5)` is `-2`.
<!-- /from -->

<!-- from:default -->
## Key points
- `/` and `%` truncate; add the divisor to a negative remainder to floor.
- Overflow wraps silently; check the result if it matters (for multiplication, `p/b != a` gives it away).
- Constant expressions are checked at compile time; variables aren't.
- `math.Round` rounds halves away from zero.
<!-- /from -->

## Common mistake
Expecting `%` to behave like a clock:

```go
hour := -1
fmt.Println(hour % 24)              // -1, not 23
fmt.Println((hour%24 + 24) % 24)    // 23
```

## In these exercises
`WrapIndex` and `SplitEvenly` need floor semantics, which Go's `/` and `%` don't give for negative numbers. `RoundPercent` can be done in integers, without `math`. `CheckedMultiply` has to spot a wrapped product, including the corner case `math.MinInt * -1`.
