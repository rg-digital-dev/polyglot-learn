# Running a server, and stopping it gracefully

`http.ListenAndServe` is fine for a demo, but it can only be stopped by ending the process, which cuts off every request in flight. A real service uses an `http.Server` and stops it with `Shutdown(ctx)`: that closes the listener, so no new connections arrive, then waits for active requests to finish, or for `ctx` to run out.

```go
package main

import (
	"context"
	"errors"
	"log"
	"net/http"
	"os"
	"os/signal"
	"time"
)

func main() {
	srv := &http.Server{Addr: "localhost:8080", Handler: http.DefaultServeMux}

	// Ctrl+C cancels ctx.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	go func() {
		if err := srv.ListenAndServe(); !errors.Is(err, http.ErrServerClosed) {
			log.Fatal(err)
		}
	}()

	<-ctx.Done()
	shutdown, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	if err := srv.Shutdown(shutdown); err != nil {
		log.Fatal(err)
	}
	log.Print("stopped cleanly")
}
```

`Serve` and `ListenAndServe` always return an error; after `Shutdown` it's `http.ErrServerClosed`, which means "stopped as asked", not failure.

Taking a `net.Listener` instead of an address makes the server easy to test: a test listens on `127.0.0.1:0`, the system picks a free port, and the test knows the address from `ln.Addr()`.

<!-- from:typescript -->
## Coming from TypeScript
- `srv.Shutdown(ctx)` is Node's `server.close()`, which also stops accepting and waits for open requests, but with a deadline built in through the context.
- `signal.NotifyContext` is `process.on("SIGINT", …)` turned into a context you can select on.
<!-- /from -->

<!-- from:default -->
## Key points
- Serve in a goroutine and wait on the context, so `main` (or `Run`) decides when to stop.
- Give `Shutdown` its own context with a timeout: the one that just told you to stop is already cancelled.
- `http.ErrServerClosed` from `Serve` is the normal end after `Shutdown`.
<!-- /from -->

## Common mistake
Passing the cancelled `ctx` to `Shutdown`. It returns `context.Canceled` at once and drops the requests in flight, which is exactly what a graceful shutdown is meant to avoid.

## In this stage
Implement `Run(ctx, ln, h)` in `run.go`: serve `h` on `ln` until `ctx` is done, then shut down gracefully, giving requests in flight up to 5 seconds, and return nil. The comment in `run.go` shows a `main` that puts every stage together; try it with `go run`, and `curl` your own shortener.
