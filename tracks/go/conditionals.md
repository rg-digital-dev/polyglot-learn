# Conditionals in Go

`if` in Go needs a `bool` condition, written without parentheses, and the braces are mandatory. An `if` can start with a short statement, such as `if n := len(s); n > 3 { ... }`, whose variable is scoped to the `if` and its `else` branches. `switch` has no fall-through by default, and a `switch` with no value acts as a tidy chain of `if`/`else if`. Boolean operators are `&&`, `||` and `!`, and they short-circuit.

```go
func ShippingCost(weightKg int, express bool) int {
	var base int
	switch {
	case weightKg <= 2:
		base = 5
	case weightKg <= 10:
		base = 9
	default:
		base = 20
	}
	if express {
		return base * 2
	}
	return base
}

func IsWeekend(day int) bool {
	return day == 6 || day == 7
}
```

<!-- from:typescript -->
## Coming from TypeScript
- No truthiness: `if count { }` does not compile. Write `if count != 0 { }`.
- There is no `? :` ternary; use an `if` with two `return`s or assign in each branch.
- `==` compares values of the same type; there is no `===` and no implicit conversion.
- `switch` cases do not fall through, so no `break` is needed.
- `%` keeps the sign of the left operand, like JS: `-7 % 4` is `-3`.
<!-- /from -->

<!-- from:default -->
## Key points
- Conditions must be `bool`; numbers and strings are never implicitly converted.
- `switch { case cond: ... }` is the idiomatic form for a list of conditions.
- A function that returns `bool` can often be a single `return` of a combined expression.
<!-- /from -->

## Common mistake
Putting `else` on its own line. Go inserts a semicolon after the closing brace, so this is a syntax error:

```go
if ok {
	return 1
}
else {        // syntax error: unexpected keyword else
	return 2
}
```

Keep it on the same line: `} else {`. Running `gofmt` catches layout issues like this.

## In this exercise
Implement `func LeapYear(year int) bool`. The whole body can be a single `return` statement.
