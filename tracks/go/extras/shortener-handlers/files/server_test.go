// Part of the URL shortener project (tracks/go/extras/shortener-handlers).
// Don't edit this file: `learn` overwrites it. Your code goes in server.go.

package shortener

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestServer(t *testing.T) {
	t.Run("c09_post_links_creates_a_link", func(t *testing.T) {
		defer failOnPanic(t)
		h := NewServer(NewStore(newClock().Now))
		rec := send(h, "POST", "/links", `{"url": "https://go.dev"}`)
		if rec.Code != http.StatusCreated {
			t.Fatalf("status %d, want 201 (body %q)", rec.Code, rec.Body)
		}
		var got map[string]string
		if err := json.Unmarshal(rec.Body.Bytes(), &got); err != nil {
			t.Fatalf("body %q isn't JSON: %v", rec.Body, err)
		}
		if got["code"] != "1" || got["url"] != "https://go.dev" {
			t.Fatalf("body %v, want code \"1\" and url \"https://go.dev\"", got)
		}
	})
	t.Run("c10_responses_are_json", func(t *testing.T) {
		defer failOnPanic(t)
		h := NewServer(NewStore(newClock().Now))
		for _, rec := range []*httptest.ResponseRecorder{
			send(h, "POST", "/links", `{"url": "https://go.dev"}`),
			send(h, "POST", "/links", `{"url": "ftp://nope"}`),
		} {
			if ct := rec.Header().Get("Content-Type"); !strings.HasPrefix(ct, "application/json") {
				t.Errorf("status %d has Content-Type %q, want application/json", rec.Code, ct)
			}
		}
	})
	t.Run("c11_a_body_that_isn_t_json_is_a_400", func(t *testing.T) {
		defer failOnPanic(t)
		h := NewServer(NewStore(newClock().Now))
		rec := send(h, "POST", "/links", `url=https://go.dev`)
		if rec.Code != http.StatusBadRequest {
			t.Fatalf("status %d, want 400", rec.Code)
		}
		if msg := errorOf(t, rec); msg == "" {
			t.Fatalf("body %q has no \"error\"", rec.Body)
		}
	})
	t.Run("c12_an_invalid_url_is_a_400_saying_so", func(t *testing.T) {
		defer failOnPanic(t)
		h := NewServer(NewStore(newClock().Now))
		rec := send(h, "POST", "/links", `{"url": "javascript:alert(1)"}`)
		if rec.Code != http.StatusBadRequest {
			t.Fatalf("status %d, want 400", rec.Code)
		}
		if msg := errorOf(t, rec); msg != "invalid URL" {
			t.Fatalf("error %q, want \"invalid URL\"", msg)
		}
	})
	t.Run("c13_a_code_redirects_to_its_url", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, _ := s.Add("https://go.dev/doc")
		rec := send(NewServer(s), "GET", "/"+code, "")
		if rec.Code != http.StatusFound {
			t.Fatalf("status %d, want 302", rec.Code)
		}
		if loc := rec.Header().Get("Location"); loc != "https://go.dev/doc" {
			t.Fatalf("Location %q, want \"https://go.dev/doc\"", loc)
		}
	})
	t.Run("c14_an_unknown_code_is_a_404", func(t *testing.T) {
		defer failOnPanic(t)
		rec := send(NewServer(NewStore(newClock().Now)), "GET", "/nope", "")
		if rec.Code != http.StatusNotFound {
			t.Fatalf("status %d, want 404", rec.Code)
		}
	})
}

// send runs one request through h, in memory.
func send(h http.Handler, method, path, body string) *httptest.ResponseRecorder {
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(method, path, strings.NewReader(body)))
	return rec
}

// errorOf reads {"error": "..."} from a response.
func errorOf(t *testing.T, rec *httptest.ResponseRecorder) string {
	t.Helper()
	var body struct {
		Error string `json:"error"`
	}
	if err := json.Unmarshal(rec.Body.Bytes(), &body); err != nil {
		t.Fatalf("body %q isn't JSON: %v", rec.Body, err)
	}
	return body.Error
}
