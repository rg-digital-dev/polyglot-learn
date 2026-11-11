// Part of the URL shortener project (tracks/go/extras/shortener-middleware).
// Don't edit this file: `learn` overwrites it. Your code goes in middleware.go.

package shortener

import (
	"bytes"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestMiddleware(t *testing.T) {
	t.Run("c21_chain_runs_the_first_middleware_outermost", func(t *testing.T) {
		defer failOnPanic(t)
		var order []string
		mark := func(name string) Middleware {
			return func(next http.Handler) http.Handler {
				return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					order = append(order, name+" in")
					next.ServeHTTP(w, r)
					order = append(order, name+" out")
				})
			}
		}
		h := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { order = append(order, "handler") })
		send(Chain(h, mark("a"), mark("b")), "GET", "/", "")
		if got := strings.Join(order, ", "); got != "a in, b in, handler, b out, a out" {
			t.Fatalf("ran %q, want \"a in, b in, handler, b out, a out\"", got)
		}
	})
	t.Run("c22_a_request_without_an_id_gets_one", func(t *testing.T) {
		defer failOnPanic(t)
		var seen string
		h := RequestID(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { seen = r.Header.Get("X-Request-ID") }))
		first := send(h, "GET", "/", "").Header().Get("X-Request-ID")
		second := send(h, "GET", "/", "").Header().Get("X-Request-ID")
		if !strings.HasPrefix(first, "req-") || !strings.HasPrefix(second, "req-") || first == second {
			t.Fatalf("response IDs %q and %q, want two different \"req-…\" IDs", first, second)
		}
		if seen != second {
			t.Fatalf("the handler saw request ID %q, want %q (the same as the response)", seen, second)
		}
	})
	t.Run("c23_an_incoming_request_id_is_kept", func(t *testing.T) {
		defer failOnPanic(t)
		h := RequestID(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {}))
		req := httptest.NewRequest("GET", "/", nil)
		req.Header.Set("X-Request-ID", "from-the-proxy")
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		if got := rec.Header().Get("X-Request-ID"); got != "from-the-proxy" {
			t.Fatalf("response ID %q, want \"from-the-proxy\"", got)
		}
	})
	t.Run("c24_logger_writes_method_path_and_status", func(t *testing.T) {
		defer failOnPanic(t)
		var log bytes.Buffer
		h := Logger(&log)(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusCreated)
		}))
		send(h, "POST", "/links", "")
		if log.String() != "POST /links 201\n" {
			t.Fatalf("logged %q, want \"POST /links 201\\n\"", log.String())
		}
	})
	t.Run("c25_logger_counts_a_plain_write_as_200", func(t *testing.T) {
		defer failOnPanic(t)
		var log bytes.Buffer
		h := Logger(&log)(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			io.WriteString(w, "hello")
		}))
		rec := send(h, "GET", "/hello", "")
		if log.String() != "GET /hello 200\n" {
			t.Fatalf("logged %q, want \"GET /hello 200\\n\"", log.String())
		}
		if rec.Body.String() != "hello" {
			t.Fatalf("body %q, want \"hello\": the logger must pass writes through", rec.Body)
		}
	})
	t.Run("c26_recover_turns_a_panic_into_a_500", func(t *testing.T) {
		defer failOnPanic(t)
		h := Recover(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { panic("boom") }))
		rec := send(h, "GET", "/", "")
		if rec.Code != http.StatusInternalServerError {
			t.Fatalf("status %d, want 500", rec.Code)
		}
		if msg := errorOf(t, rec); msg != "internal error" {
			t.Fatalf("error %q, want \"internal error\"", msg)
		}
	})
	t.Run("c27_recover_leaves_other_responses_alone", func(t *testing.T) {
		defer failOnPanic(t)
		h := Recover(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusTeapot)
			io.WriteString(w, "short and stout")
		}))
		rec := send(h, "GET", "/", "")
		if rec.Code != http.StatusTeapot || rec.Body.String() != "short and stout" {
			t.Fatalf("got %d %q, want 418 \"short and stout\"", rec.Code, rec.Body)
		}
	})
}
