# select, timeouts and context

`select` waits on several channel operations at once and runs the case of whichever is ready first; if several are ready, it picks one at random. Combined with a timer channel, that gives a timeout: `time.After(d)` returns a channel that receives a value once `d` has passed. The same pattern works for cancellation with `context.Context`: `ctx.Done()` is a channel that is closed when the context is cancelled or its deadline passes, and `ctx.Err()` then says which of the two happened. Run the slow work in a goroutine that sends its result on a channel, and `select` between that channel and the timer or `ctx.Done()`.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

// FirstTick waits for the first value from ticks, but gives up when ctx ends.
func FirstTick(ctx context.Context, ticks <-chan string) (string, error) {
	select {
	case t := <-ticks:
		return t, nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

func main() {
	ticks := make(chan string, 1)
	go func() {
		time.Sleep(time.Second)
		ticks <- "tick" // too late
	}()

	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel() // always release the context's timer

	_, err := FirstTick(ctx, ticks)
	fmt.Println(err) // context deadline exceeded

	select {
	case t := <-ticks:
		fmt.Println(t)
	case <-time.After(2 * time.Second):
		fmt.Println("no tick")
	}
	// tick
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `select` is close to `Promise.race`, but over channels, and it doesn't start anything: the work must already be running in a goroutine.
- `time.After(d)` plays the role of a `setTimeout` promise. `context.Context` is like an `AbortSignal`: `ctx.Done()` fires on abort, `ctx.Err()` gives the reason.
- Losing the race doesn't stop the work. Go can't kill a goroutine; the slow work keeps running until it returns, so make sure it can still finish its send.
<!-- /from -->

<!-- from:default -->
## Key points
- `select` blocks until one case can proceed; with no `default` case it never busy-waits.
- Give the result channel a buffer of 1, so the worker can send and exit even after the timeout won.
- `ctx.Err()` is `context.Canceled` after `cancel()` and `context.DeadlineExceeded` after a deadline; compare with `errors.Is`.
- Call the `cancel` function returned by `context.WithTimeout` or `WithCancel`, usually with `defer`.
<!-- /from -->

## Common mistake
An unbuffered result channel leaks the worker goroutine when the timeout wins:

```go
done := make(chan int)            // nobody will ever receive after a timeout,
go func() { done <- work() }()    // so this goroutine blocks on its send forever
done := make(chan int, 1)         // fixed: the send succeeds, the goroutine exits
```

There is no error message for this: the program just keeps the stuck goroutines, and their memory, until it exits.

## In this exercise
Implement `WithTimeout` (using `time.After`) and `WithContext` (using `ctx.Done()`) in `withtimeout.go`. The tests use generous timings: fast work takes 20 ms, slow work takes seconds, and "right away" means under a second.
