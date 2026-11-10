# Lists, Maps and Tuples in Go

Go's list is the slice: `[]int{1, 2, 3}`, grown with `append`, which returns a new slice you assign back. A map is `map[string]int{"alice": 3}`. Reading a missing key gives the zero value, so use `v, ok := m[k]` to tell "missing" from "zero". Fixed-size arrays (`[3]int`) exist but are rare. Go has no tuple type. Group related values in a `struct`, or return several values from a function. Counting starts at 0.

```go
package main

import "fmt"

func main() {
	xs := []int{1, 2, 3}
	xs = append(xs, 4)
	fmt.Println(xs, len(xs), xs[0], xs[1:3]) // --> [1 2 3 4] 4 1 [2 3]
	m := map[string]int{"alice": 3}
	m["bob"] = 5
	v, ok := m["zed"]
	fmt.Println(m["alice"], v, ok) // --> 3 0 false
	delete(m, "bob")
	type Pair struct {
		Name  string
		Score int
	}
	p := Pair{"alice", 3}
	fmt.Println(p, p.Name) // --> {alice 3} alice
	b := xs
	b[0] = 99
	fmt.Println(xs[0]) // --> 99
}
```

<!-- from:python -->
## Coming from Python
- There are no negative indexes. The last item is `xs[len(xs)-1]`.
- Appending is `xs = append(xs, 4)`, not a method. Forgetting the assignment loses the item.
- Slices share memory: `b := xs` then `b[0] = 99` changes `xs` too.
- A missing key does not raise. You get `0`, `""` or `false`.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- A slice or map holds one element type, and the compiler enforces it.
- There is no `push`, `.length` or `xs[-1]`. Use `append`, `len(xs)` and `xs[len(xs)-1]`.
- A map is not an object: write `m["alice"]`, not `m.alice`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Indexes start at 0, not 1.
- Lists and maps are separate types. A slice has no string keys, and a map has no order.
<!-- /from -->

<!-- from:default -->
## Key points
- A slice is `[]T`; `len(xs)` is its length; `xs[i]` reads and `xs[a:b]` slices.
- `append` returns the new slice, so assign it back.
- `v, ok := m[k]` tells you whether the key exists.
- A nil map can be read, but writing to it panics. Create one with a literal or `make`.
<!-- /from -->

## Common mistake
Writing to a nil map: `var m map[string]int` then `m["a"] = 1` panics. Start with `m := map[string]int{}`.
