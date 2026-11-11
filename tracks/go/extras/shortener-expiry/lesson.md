# Background goroutines: tickers and stopping

Services often have work that isn't a request: cleaning caches, retrying, sending batches. In Go that's a goroutine with a loop. `time.NewTicker(d)` delivers the time on its channel `C` every `d`, and a `select` that also listens on a *quit* channel gives the loop a way to stop. Every background goroutine should have such a way out, or it leaks: it keeps running, and holding memory, until the program ends.

```go
package main

import (
	"fmt"
	"sync"
	"time"
)

// Every runs f every d until stop is called.
func Every(d time.Duration, f func()) (stop func()) {
	ticker := time.NewTicker(d)
	quit := make(chan struct{})
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ticker.C:
				f()
			case <-quit:
				return
			}
		}
	}()
	var once sync.Once
	return func() { once.Do(func() { close(quit) }) } // closing twice would panic
}

func main() {
	stop := Every(100*time.Millisecond, func() { fmt.Println("tick") })
	time.Sleep(350 * time.Millisecond)
	stop()
	stop() // fine, thanks to sync.Once
}
```

Closing a channel is a broadcast: every receiver sees it at once, and keeps seeing it. That makes `close(quit)` the usual "stop" signal.

The expiry itself follows the store's clock, not the ticker. A link is gone from the moment its time is up, even if the janitor hasn't run yet; the janitor only frees the memory.

<!-- from:typescript -->
## Coming from TypeScript
- `NewTicker` is `setInterval`, and stop is `clearInterval`. The difference is that the work runs in a goroutine at the same time as your requests, so it needs the store's lock like any request.
- Returning a `stop func()` is the same idea as returning an unsubscribe function from a subscription.
<!-- /from -->

<!-- from:default -->
## Key points
- Every goroutine you start needs a way to end: a quit channel, a context, or its input channel closing.
- `defer ticker.Stop()` in the goroutine releases the ticker when it exits.
- `sync.Once` makes "stop" safe to call any number of times.
- Deleting from a map while ranging over it is allowed in Go.
<!-- /from -->

## Common mistake
Checking expiry only in the janitor: between ticks, an expired link would still redirect. Decide "live or not" in one helper that `Get`, `Visit` and `Expire` all use.

## In this stage
Implement `AddTTL`, `Expire` and `StartJanitor` in `expiry.go`, and make `Get` and `Visit` treat an expired link as `ErrNotFound`. A link expires when the store's clock reaches its creation time plus the TTL; links from `Add` never expire.
