# Goroutines in Go

The `go` keyword starts a function call in a new goroutine, a lightweight thread managed by the Go runtime, and returns immediately. Goroutines run concurrently and, on a multi-core machine, in parallel. Something has to wait for them: a `sync.WaitGroup` counts running goroutines (`Add` before starting, `Done` when finished, `Wait` to block until the count is zero), and channels pass values between goroutines. Two goroutines must never write the same variable without synchronisation, but writing to different elements of a slice is fine.

```go
import (
	"strings"
	"sync"
)

// CountBoth counts words and lines at the same time.
func CountBoth(text string) (words, lines int) {
	var wg sync.WaitGroup
	wg.Add(2)
	go func() {
		defer wg.Done()
		words = len(strings.Fields(text))
	}()
	go func() {
		defer wg.Done()
		lines = strings.Count(text, "\n") + 1
	}()
	wg.Wait() // blocks until both goroutines have called Done
	return words, lines
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `Promise.all` interleaves tasks on one thread; goroutines can really run in parallel.
- There is no `await`: a blocking call such as `time.Sleep` only blocks its own goroutine.
- `wg.Wait()` plays the role of awaiting `Promise.all`, but it returns no values; the goroutines store them.
- A goroutine has no return value; results go into a variable, a slice element or a channel.
<!-- /from -->

<!-- from:default -->
## Key points
- Call `wg.Add` before the `go` statement, and `defer wg.Done()` inside the goroutine.
- Allocate the result slice first with `make([]int, len(input))` and let each goroutine fill its own index.
- Since Go 1.22 each loop iteration has its own variable, so a goroutine can use the loop index directly.
<!-- /from -->

## Common mistake
Appending to a shared slice from every goroutine:

```go
go func() { defer wg.Done(); results = append(results, slow(j)) }()
// a data race, and results arrive in finishing order: [300 100 200] comes back as [100 200 300]
go func() { defer wg.Done(); results[i] = slow(j) }()   // fixed: each goroutine owns one index
```

## In this exercise
Implement `func RunAll(delays []int) []int` using the provided `task(ms)`; do not change `task`. The tests check that the call takes about as long as the longest delay, not the sum.
