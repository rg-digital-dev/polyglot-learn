# Generics

A function or type can take **type parameters** in square brackets, so one definition works for many types: `func Last[T any](items []T) T`. Each type parameter has a **constraint**, an interface that says what the code may do with it. `any` allows nothing but storing and passing values. `comparable` allows `==` and `!=`, which map keys need. `cmp.Ordered` (from the `cmp` package) allows `<` and `>`, which sorting needs. The compiler usually infers the type arguments from the arguments you pass, so callers write `Last(names)`, not `Last[string](names)`. A generic type, such as `type Stack[T any] struct{ ... }`, carries its parameter into its methods, but a method can't add type parameters or constraints of its own; use a plain function for that.

```go
package main

import (
	"cmp"
	"fmt"
)

// Counter counts how often each key was seen.
type Counter[K comparable] struct {
	counts map[K]int
}

func NewCounter[K comparable]() *Counter[K] {
	return &Counter[K]{counts: map[K]int{}}
}

func (c *Counter[K]) See(k K) { c.counts[k]++ }

// MostSeen needs to break ties by key, so it asks for cmp.Ordered:
// a function, because a method can't make K's constraint stricter.
func MostSeen[K cmp.Ordered](c *Counter[K]) (best K) {
	for k, n := range c.counts {
		if n > c.counts[best] || (n == c.counts[best] && k < best) {
			best = k
		}
	}
	return best
}

func main() {
	c := NewCounter[string]() // nothing to infer from, so say the type
	for _, w := range []string{"b", "a", "b", "a", "c"} {
		c.See(w)
	}
	fmt.Println(MostSeen(c)) // a (inferred: K is string)
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `[T any]` is `<T>`. Constraints are interfaces, like `<T extends ...>`, but they can also list operators through type sets such as `cmp.Ordered`.
- Go generics exist at run time too: there is no type erasure trick, and no `as` cast is needed to get a `T` back out.
- There are no generic methods. `func (s *Set[T]) Map[U any](...)` doesn't compile; write `func Map[T, U any](...)` instead.
- `comparable` is the closest thing to "can be a map key". Slices, maps and functions are not comparable.
<!-- /from -->

<!-- from:default -->
## Key points
- Pick the loosest constraint the code needs: `any`, then `comparable` for `==`, then `cmp.Ordered` for `<`.
- Type arguments are inferred from the arguments; write them only when there is nothing to infer from.
- `map[T]struct{}` is the usual way to store a set: the empty struct takes no memory.
- A method uses its type's parameters; anything needing more goes in a generic function.
<!-- /from -->

## Common mistake
Asking a `comparable` type parameter for an order:

```go
func Smallest[T comparable](items []T) T {
	best := items[0]
	for _, v := range items {
		if v < best { // invalid operation: v < best (type parameter T cannot use operator <)
			best = v
		}
	}
	return best
}
```

`comparable` only promises `==`. Constrain `T` with `cmp.Ordered` instead.

## In this exercise
Implement the `Set[T comparable]` methods, `New`, `Sorted`, `Map` and `Filter` in `genericset.go`. The zero `Set` must work, so `Add` creates the map when it is nil. The tests use sets of ints, strings and a small struct.
