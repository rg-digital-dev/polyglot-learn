# Sharing in Go

Go copies on assignment, but some values *contain* a pointer. A slice is a small header (pointer, length, capacity) over a backing array: `b := a` copies the header, so both slices see the same elements. Maps work the same way: `m2 := m` is the same map. Arrays (`[3]int`) and structs without slices inside are copied in full. To copy a slice's elements, use `slices.Clone(a)` or `append([]int(nil), a...)`; `maps.Clone(m)` copies a map, one level deep.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	a := [][]int{{1, 2}, {3}}
	b := a                  // the same backing array
	c := slices.Clone(a)    // a new outer slice, the same inner slices
	d := make([][]int, len(a))
	for i, row := range a {
		d[i] = slices.Clone(row) // a deep copy, one row at a time
	}

	a[0][0] = 9
	fmt.Println(b[0], c[0], d[0]) // [9 2] [9 2] [1 2]
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Slices and maps behave like arrays and objects: assigning shares them.
- Arrays (`[3]int`) and structs are copied by `=`, like numbers.
- `slices.Clone` is `[...a]`; there's no `structuredClone`, so deep copies are loops.
- `append` may write into the shared backing array when there's spare capacity, which is a Go-only surprise.
<!-- /from -->

<!-- from:default -->
## Key points
- A slice copy shares the backing array; a map copy is the same map.
- `slices.Clone` and `maps.Clone` copy one level.
- Arrays and plain structs are copied by assignment.
<!-- /from -->

## Common mistake
Reusing one slice for every row:

```go
row := make([]int, 3)
grid := [][]int{row, row} // two headers over one array
grid[0][0] = 9            // grid is [[9 0 0] [9 0 0]]

for i := range grid {     // fixed: make a new slice per row
	grid[i] = make([]int, 3)
}
```

## In these exercises
`MakeGrid(rows, cols, fill)` makes a new slice per row. `AddBonus(scores, name, points)` returns a new map with every slice cloned: copying the map but keeping the slices shares their backing arrays with `scores`, and `append` can then write into them.
