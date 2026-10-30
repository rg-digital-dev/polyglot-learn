# Collections and sorting in Go

A slice, written `[]T`, is a growable view onto an underlying array. Copying a slice variable copies only the view, so two slices can share the same elements. The `slices` package (Go 1.21+) does the common work: `slices.Sort` sorts in place in ascending order, `slices.SortFunc` takes a comparison that returns a negative, zero or positive `int`, and `slices.Clone` makes an independent copy. The older `sort` package still works but is rarely needed in new code.

```go
import (
	"cmp"
	"slices"
)

// LongestFirst returns the words sorted by length, longest first,
// without changing the caller's slice.
func LongestFirst(words []string) []string {
	out := slices.Clone(words)
	slices.SortFunc(out, func(a, b string) int {
		return cmp.Compare(len(b), len(a))
	})
	return out
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `slices.Sort` compares numbers as numbers, unlike the default `Array.prototype.sort`.
- Sorting works in place and returns nothing, like `sort()`, so copy first if the caller's data must stay intact.
- A comparison returning `cmp.Compare(b, a)` gives descending order, much like `(a, b) => b - a`.
- `s[:n]` panics if `n > len(s)`; it does not clamp like `slice(0, n)`. Use `s[:min(n, len(s))]`.
- Passing a slice to a function shares its elements, so sorting a parameter changes the caller's data.
<!-- /from -->

<!-- from:default -->
## Key points
- `slices.Clone` (or `append([]int(nil), s...)`) copies before you sort.
- `slices.Sort` for ascending order; `slices.SortFunc` with `cmp.Compare` for anything else.
- `min` is a built-in since Go 1.21 and is handy for clamping slice bounds.
<!-- /from -->

## Common mistake
Sorting the input slice directly:

```go
slices.Sort(scores)                // the caller's slice is now reordered
sorted := slices.Clone(scores)     // fixed: sort a copy
slices.Sort(sorted)
```

## In this exercise
Implement `func TopScores(scores []int, n int) []int`, highest first, leaving `scores` unchanged. Returning `nil` or an empty slice are treated the same by the tests.
