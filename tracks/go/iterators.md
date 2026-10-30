# Iteration pipelines in Go

Go has no `map`/`filter` chain in everyday code. A pipeline is a plain `for ... range` loop that builds a slice, followed by a call into the `slices` package. To sort by several keys, give `slices.SortFunc` a comparison that checks the first key and falls back to the next one only when they are equal; `cmp.Or` (Go 1.22+) returns the first non-zero comparison, which makes that a one-liner. A map has no order, so turn it into a slice of pairs before sorting. Go 1.23 added the `iter` package and `range` over iterator functions, but the exercise module declares `go 1.22`, so stick to slices here.

```go
import (
	"cmp"
	"slices"
	"strings"
)

// LongestWords returns up to n distinct words of three or more letters,
// longest first, ties in alphabetical order.
func LongestWords(text string, n int) []string {
	seen := map[string]bool{}
	var words []string
	for _, w := range strings.Fields(strings.ToLower(text)) {
		if len(w) > 2 && !seen[w] {
			seen[w] = true
			words = append(words, w)
		}
	}
	slices.SortFunc(words, func(a, b string) int {
		return cmp.Or(cmp.Compare(len(b), len(a)), cmp.Compare(a, b))
	})
	return words[:min(n, len(words))]
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Where TS writes `.filter(...).map(...)`, Go writes one loop with an `if` and an `append`.
- `Object.entries(counts)` becomes a loop over the map that appends `struct{ word string; count int }` values.
- The comparison returns an `int`, like a TS comparator, and `cmp.Compare` saves writing `a - b`.
- `s[:n]` panics if `n > len(s)`, so clamp with `min(n, len(s))`.
<!-- /from -->

<!-- from:default -->
## Key points
- Collect into a slice with `range` and `append`, then sort with `slices.SortFunc`.
- `cmp.Or(first, second)` chains comparisons by priority.
- A small local struct type is the usual way to keep a word and its count together.
<!-- /from -->

## Common mistake
Sorting by count alone:

```go
slices.SortFunc(pairs, func(a, b pair) int { return cmp.Compare(b.count, a.count) })
// equal counts come out in map order, which changes from run to run
return cmp.Or(cmp.Compare(b.count, a.count), cmp.Compare(a.word, b.word)) // fixed
```

## In this exercise
Implement `func TopWords(text string, n int) []string`: count the words, sort by count descending then word ascending, and return the first `n` words. Returning `nil` or an empty slice are treated the same.
