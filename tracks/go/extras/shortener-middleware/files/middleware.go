package shortener

import (
	"io"
	"net/http"
)

// Middleware wraps a handler with behaviour of its own.
type Middleware func(http.Handler) http.Handler

// Chain wraps h in each middleware, so that the first in the list is the
// outermost: Chain(h, a, b) behaves like a(b(h)).
func Chain(h http.Handler, m ...Middleware) http.Handler {
	panic("TODO: implement Chain")
}

// RequestID keeps the request's X-Request-ID header if it has one, or makes a
// new ID ("req-1", "req-2", …; safe to call from many goroutines), sets it on
// the request so later handlers see it, and on the response.
func RequestID(next http.Handler) http.Handler {
	panic("TODO: implement RequestID")
}

// Logger writes one line to out per request: method, path and the status
// sent, as in "POST /links 201". A handler that writes a body without calling
// WriteHeader sent a 200.
func Logger(out io.Writer) Middleware {
	panic("TODO: implement Logger")
}

// Recover turns a panic in next into a 500 with {"error": "internal error"},
// and leaves every other response alone.
func Recover(next http.Handler) http.Handler {
	panic("TODO: implement Recover")
}
