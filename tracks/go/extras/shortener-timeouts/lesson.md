# Deadlines with context

Every incoming request carries a `context.Context` (`r.Context()`). It's cancelled when the client goes away, and you can derive a child with a deadline: `context.WithTimeout(r.Context(), d)`. Code that does slow work, such as a database query, an HTTP call to another service, or a loop, watches `ctx.Done()` and stops when it's closed. That's how Go stops work nobody is waiting for any more.

A handler can't be interrupted from outside, though. To answer *on time* while the handler is still busy, run it in a goroutine and `select` between "it finished" and "the deadline passed":

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func lookup(ctx context.Context, key string) (string, error) {
	select {
	case <-time.After(2 * time.Second): // pretend the database is slow
		return "value of " + key, nil
	case <-ctx.Done():
		return "", ctx.Err() // context.DeadlineExceeded or context.Canceled
	}
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel() // always release the timer

	done := make(chan string, 1) // buffered: the goroutine can always finish its send
	go func() {
		v, err := lookup(ctx, "answer")
		if err != nil {
			v = "gave up: " + err.Error()
		}
		done <- v
	}()

	select {
	case v := <-done:
		fmt.Println(v)
	case <-ctx.Done():
		fmt.Println("too slow:", ctx.Err())
	}
}
```

For HTTP there's one more rule: an `http.ResponseWriter` may only be used from one goroutine. So the handler writes into a buffer of its own, and only the middleware touches the real writer, copying the buffer if the handler made it in time, or writing the 503 if it didn't.

<!-- from:typescript -->
## Coming from TypeScript
- `context.Context` is close to an `AbortSignal`: `ctx.Done()` is the abort event, `ctx.Err()` the reason, and `WithTimeout` is `AbortSignal.timeout()`. Contexts also chain: cancelling a parent cancels its children.
- The goroutine plus `select` is `Promise.race([work, timeout])`. As with promises, losing the race doesn't stop the work; the context is how you tell it to stop.
<!-- /from -->

<!-- from:default -->
## Key points
- Derive from `r.Context()`, not `context.Background()`, so a client that disconnects cancels the work too.
- `defer cancel()` right after `WithTimeout`, every time.
- Pass the derived context on with `r.WithContext(ctx)`, or the handler never sees the deadline.
- One goroutine per `ResponseWriter`: buffer what the handler writes.
<!-- /from -->

## Common mistake
`http.TimeoutHandler` exists, and it's a fine choice in real code, but it answers with an HTML body. Read its source in `net/http` after this stage; it's the same buffer-and-select design.

## In this stage
Implement `Timeout(d)` in `timeout.go`, using the `buffered` type for the handler's response. A handler that finishes within `d` gets its headers, status and body through unchanged; otherwise the client gets 503 and `{"error": "timed out"}` soon after `d`, and the handler's context is cancelled.
