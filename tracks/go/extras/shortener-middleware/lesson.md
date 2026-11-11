# Middleware: handlers that wrap handlers

Cross-cutting work such as logging, request IDs and recovering from panics doesn't belong in every handler. In Go it goes in *middleware*: a function that takes a handler and returns a new one, which does its own work around a call to the original. Because both are `http.Handler`s, they stack in any order.

```go
package main

import (
	"log"
	"net/http"
	"time"
)

type Middleware func(http.Handler) http.Handler

func Timing(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		next.ServeHTTP(w, r) // the wrapped handler runs here
		log.Printf("%s %s took %v", r.Method, r.URL.Path, time.Since(start))
	})
}

// A middleware that needs settings is a function returning a Middleware.
func Header(key, value string) Middleware {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set(key, value)
			next.ServeHTTP(w, r)
		})
	}
}

func main() {
	hello := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { w.Write([]byte("hi")) })
	h := Timing(Header("X-Powered-By", "Go")(hello)) // Timing is outermost
	http.ListenAndServe("localhost:8080", h)
}
```

To learn what a handler did, such as its status, give it a writer of your own: a struct that *embeds* `http.ResponseWriter` gets all its methods, and you override only `WriteHeader` to remember the status.

<!-- from:typescript -->
## Coming from TypeScript
- This is Express or Koa middleware without `next()` as a callback: calling `next.ServeHTTP(w, r)` is the `await next()`, and code after it runs on the way out.
- Embedding (`type statusWriter struct { http.ResponseWriter; status int }`) is like extending a class and overriding one method, except the "base" is any value that implements the interface.
- `recover()` inside a deferred function is the `catch` for a panic; it only works in the goroutine that panicked.
<!-- /from -->

<!-- from:default -->
## Key points
- A middleware is `func(http.Handler) http.Handler`; one with settings is a function that returns one.
- `Chain(h, a, b)` should mean `a(b(h))`: wrap from the last to the first so the first listed is outermost.
- `defer` + `recover()` must run in the same goroutine as the panic, at the top of the wrapping handler.
<!-- /from -->

## Common mistake
Recording the status only in `WriteHeader`: a handler that just calls `Write` never calls your `WriteHeader`, and the logger prints 0. Treat "wrote a body without a status" as 200.

## In this stage
Implement `Chain`, `RequestID`, `Logger` and `Recover` in `middleware.go`. Request IDs are "req-1", "req-2", … unless the request brought an `X-Request-ID`; set it on both the request and the response. The logger writes `METHOD PATH STATUS` and a newline per request. `Recover` answers a panic with 500 and `{"error": "internal error"}`.
