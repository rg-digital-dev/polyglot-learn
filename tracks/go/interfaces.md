# Interfaces in Go

A Go interface is a set of method signatures. A type satisfies it simply by having those methods; there is no `implements` keyword. A variable of interface type can hold any value whose type satisfies the interface, so a `[]Notifier` can mix several concrete types, and a method call on it runs the method of whatever value is stored. Methods with a value receiver belong to both `T` and `*T`; methods with a pointer receiver belong only to `*T`.

```go
type Notifier interface {
	Channel() string
	CostCents() int
}

type Email struct{ Address string }

type Sms struct {
	Number   string
	Segments int
}

func (e Email) Channel() string { return "email " + e.Address }
func (e Email) CostCents() int  { return 0 }

func (s Sms) Channel() string { return "sms " + s.Number }
func (s Sms) CostCents() int  { return 5 * s.Segments }

func TotalCost(targets []Notifier) int {
	total := 0
	for _, t := range targets {
		total += t.CostCents()
	}
	return total
}

// TotalCost([]Notifier{Email{Address: "a@example.org"}, Sms{Number: "555-0100", Segments: 2}}) == 10
```

<!-- from:typescript -->
## Coming from TypeScript
- Interfaces are satisfied implicitly, much like TS structural typing, but only methods count, not fields.
- The check happens at compile time wherever a value is assigned to the interface type.
- There are no classes; methods are declared on named types, usually structs.
- A nil interface value panics when you call a method on it, much like calling a method on `undefined`.
<!-- /from -->

<!-- from:default -->
## Key points
- Keep interfaces small; one or two methods is common in Go.
- Code that takes an interface should only use its methods, not the concrete type behind it.
- `slices.SortFunc` is not stable; use `slices.SortStableFunc` when equal elements must keep their order.
<!-- /from -->

## Common mistake
Switching a method to a pointer receiver:

```go
func (r *Rect) Area() int { return r.Width * r.Height }
shapes := []Shape{Rect{Width: 2, Height: 3}}
// cannot use Rect{…} (value of struct type Rect) as Shape value in array or slice literal:
// Rect does not implement Shape (method Area has pointer receiver)
```

Keep the value receivers from the stub; the tests build `Rect{...}` and `Square{...}` values.

## In this exercise
Fill in the `Area` and `Label` methods on `Rect` and `Square`, then implement `func DescribeAll(shapes []Shape) []string`. Copy the slice before sorting so the caller's order is untouched.
