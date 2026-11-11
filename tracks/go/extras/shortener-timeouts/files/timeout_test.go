// Part of the URL shortener project (tracks/go/extras/shortener-timeouts).
// Don't edit this file: `learn` overwrites it. Your code goes in timeout.go.
//
// Timings are loose so slow machines don't fail: the limit is 50 ms, slow
// handlers take seconds, and "in time" means well under a second.

package shortener

import (
	"io"
	"net/http"
	"strings"
	"testing"
	"time"
)

// slow waits for its request's context to end (or 3 seconds), and reports
// on cancelled when the context ended.
func slow(cancelled chan<- error) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		select {
		case <-r.Context().Done():
			cancelled <- r.Context().Err()
		case <-time.After(3 * time.Second):
			io.WriteString(w, "too late")
		}
	})
}

func TestTimeout(t *testing.T) {
	t.Run("c31_a_fast_handler_answers_as_usual", func(t *testing.T) {
		defer failOnPanic(t)
		h := Timeout(time.Second)(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set("X-Answer", "42")
			w.WriteHeader(http.StatusCreated)
			io.WriteString(w, "made it")
		}))
		rec := send(h, "GET", "/", "")
		if rec.Code != http.StatusCreated || rec.Body.String() != "made it" || rec.Header().Get("X-Answer") != "42" {
			t.Fatalf("got %d %q (X-Answer %q), want 201 \"made it\" (X-Answer 42)", rec.Code, rec.Body, rec.Header().Get("X-Answer"))
		}
	})
	t.Run("c32_a_slow_handler_gets_a_503_in_time", func(t *testing.T) {
		defer failOnPanic(t)
		h := Timeout(50 * time.Millisecond)(slow(make(chan error, 1)))
		start := time.Now()
		rec := send(h, "GET", "/", "")
		if took := time.Since(start); took > time.Second {
			t.Fatalf("answered after %v; want soon after the 50ms limit", took)
		}
		if rec.Code != http.StatusServiceUnavailable {
			t.Fatalf("status %d, want 503", rec.Code)
		}
	})
	t.Run("c33_the_slow_handler_s_context_is_cancelled", func(t *testing.T) {
		defer failOnPanic(t)
		cancelled := make(chan error, 1)
		send(Timeout(50*time.Millisecond)(slow(cancelled)), "GET", "/", "")
		select {
		case err := <-cancelled:
			if err == nil {
				t.Fatal("the context ended without an error")
			}
		case <-time.After(time.Second):
			t.Fatal("the handler's context was never cancelled: pass r.WithContext(ctx) to it")
		}
	})
	t.Run("c34_the_timeout_answer_is_a_json_error", func(t *testing.T) {
		defer failOnPanic(t)
		rec := send(Timeout(50*time.Millisecond)(slow(make(chan error, 1))), "GET", "/", "")
		if ct := rec.Header().Get("Content-Type"); !strings.HasPrefix(ct, "application/json") {
			t.Fatalf("Content-Type %q, want application/json", ct)
		}
		if msg := errorOf(t, rec); msg != "timed out" {
			t.Fatalf("error %q, want \"timed out\"", msg)
		}
	})
}
