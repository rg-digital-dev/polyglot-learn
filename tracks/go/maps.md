# Maps in Go

A Go map, written `map[K]V`, is a built-in hash table. Create one with `make(map[string]int)` or a literal such as `map[string]int{}`. Reading a missing key returns the value type's zero value (`0` for `int`), so `counts[k]++` works without checking first. To tell "missing" apart from "zero", use the two-value form `v, ok := m[k]`. Iteration order over a map is deliberately random.

```go
func Tally(colors []string) map[string]int {
	counts := make(map[string]int)
	for _, c := range colors {
		counts[c]++
	}
	return counts
}

func Describe(counts map[string]int, color string) string {
	n, ok := counts[color]
	if !ok {
		return color + ": never seen"
	}
	return fmt.Sprintf("%s: %d", color, n)
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `map[string]int` is like `Map<string, number>`, but uses index syntax: `m[k]`, `m[k] = v`, `delete(m, k)`.
- A missing key gives `0`, not `undefined`, which makes counting easy and silent typos easy too.
- `len(m)` is the number of entries.
- Maps are reference-like: passing one to a function lets that function modify it.
- Do not rely on order when ranging over a map; it changes between runs.
<!-- /from -->

<!-- from:default -->
## Key points
- Always initialize a map (`make` or a literal) before writing to it.
- `v, ok := m[k]` distinguishes a missing key from a stored zero.
- Map iteration order is random; sort the keys if you need a stable order.
<!-- /from -->

## Common mistake
Writing to a nil map panics:

```go
var counts map[string]int      // declared, but nil
counts["go"]++                  // panic: assignment to entry in nil map

counts := map[string]int{}     // fixed: an empty, usable map
counts["go"]++
```

## In this exercise
Implement `func WordCount(text string) map[string]int`. Only ASCII letters and digits form words, so `unicode.IsLetter` is too generous ("é" counts); compare against `'a'..'z'`, `'A'..'Z'` and `'0'..'9'` instead.
