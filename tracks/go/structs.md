# Structs in Go

A struct type groups named fields: `type Range struct { Low, High int }`. You build a value with a composite literal such as `Range{Low: 1, High: 9}`; any field you leave out gets its zero value. Fields and methods whose names start with a capital letter are visible outside the package. A method is a function with a receiver written before its name, and a value receiver (`func (r Range) ...`) works on a copy. Structs are values: assigning one or passing it to a function copies every field, and `==` compares them field by field.

```go
type Reading struct {
	Station string
	Celsius int
}

type Range struct {
	Low, High int
}

func (r Range) Spread() int {
	return r.High - r.Low
}

func RangeOf(readings []Reading) Range {
	if len(readings) == 0 {
		return Range{}
	}
	out := Range{Low: readings[0].Celsius, High: readings[0].Celsius}
	for _, r := range readings {
		out.Low = min(out.Low, r.Celsius)
		out.High = max(out.High, r.Celsius)
	}
	return out
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A struct is a fixed set of typed fields, like an interface plus the data; there are no classes or constructors.
- Missing fields are not `undefined`: they become `0`, `""`, `false` or `nil`.
- Assigning a struct copies it, unlike a JS object, which is shared by reference. Use a pointer (`*Range`) to share one.
- `Range{Low: 1, High: 9} == Range{Low: 1, High: 9}` is `true`.
<!-- /from -->

<!-- from:default -->
## Key points
- Prefer keyed literals (`Range{Low: 1, High: 9}`); they survive new fields being added.
- Lowercase field names are private to the package.
- `int` division truncates toward zero, which rounds down for non-negative values.
<!-- /from -->

## Common mistake
Changing the loop variable instead of the slice element:

```go
for _, r := range readings {
	r.Celsius = 0          // changes a copy; readings is untouched
}
for i := range readings {
	readings[i].Celsius = 0 // fixed: index into the slice
}
```

## In this exercise
Implement `func OrderTotal(items []LineItem, discountPercent int) OrderSummary`. Both structs are generated in your stub. Compute the discount as `SubtotalCents * discountPercent / 100`; dividing first loses cents.
