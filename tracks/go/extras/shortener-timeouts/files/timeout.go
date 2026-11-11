package shortener

import (
	"net/http"
	"time"
)

// Timeout gives next d to answer. next runs with a context that's cancelled
// after d (so its work can stop), in a goroutine of its own, writing into a
// buffer: if it finishes in time, copy its headers, status and body to w; if
// d passes first, answer 503 with {"error": "timed out"} instead.
//
// Only one goroutine may write to w: the buffer is how the handler's goroutine
// stays away from it.
func Timeout(d time.Duration) Middleware {
	panic("TODO: implement Timeout")
}

// buffered is an http.ResponseWriter that keeps what's written, for Timeout.
// Fill in its methods: Header, Write and WriteHeader.
type buffered struct {
	header http.Header
	status int
	body   []byte
}
