// Part of the URL shortener project (tracks/go/extras/shortener-stats). Don't
// edit this file: `learn` overwrites it. Your code goes in stats.go and
// server.go.

package shortener

import (
	"encoding/json"
	"errors"
	"net/http"
	"testing"
	"time"
)

func TestStats(t *testing.T) {
	t.Run("c15_visit_counts_a_hit", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, _ := s.Add("https://go.dev")
		s.Visit(code)
		link, err := s.Visit(code)
		if err != nil || link.Hits != 2 {
			t.Fatalf("second Visit = %+v, %v; want 2 hits", link, err)
		}
		if got, _ := s.Get(code); got.Hits != 2 {
			t.Fatalf("Get after two visits shows %d hits, want 2", got.Hits)
		}
	})
	t.Run("c16_visiting_an_unknown_code_is_errnotfound", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		if _, err := s.Visit("nope"); !errors.Is(err, ErrNotFound) {
			t.Fatalf("Visit(\"nope\") error = %v, want ErrNotFound", err)
		}
	})
	t.Run("c17_get_doesn_t_count_a_hit", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, _ := s.Add("https://go.dev")
		s.Get(code)
		if got, _ := s.Get(code); got.Hits != 0 {
			t.Fatalf("%d hits after two Gets, want 0", got.Hits)
		}
	})
	t.Run("c18_a_redirect_counts_a_hit", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, _ := s.Add("https://go.dev")
		h := NewServer(s)
		send(h, "GET", "/"+code, "")
		send(h, "GET", "/"+code, "")
		if got, _ := s.Get(code); got.Hits != 2 {
			t.Fatalf("%d hits after two redirects, want 2", got.Hits)
		}
	})
	t.Run("c19_get_links_code_shows_the_link_as_json", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		code, _ := s.Add("https://go.dev")
		h := NewServer(s)
		send(h, "GET", "/"+code, "")
		rec := send(h, "GET", "/links/"+code, "")
		if rec.Code != http.StatusOK {
			t.Fatalf("status %d, want 200 (body %q)", rec.Code, rec.Body)
		}
		var got struct {
			Code    string `json:"code"`
			URL     string `json:"url"`
			Hits    int    `json:"hits"`
			Created string `json:"created"`
		}
		if err := json.Unmarshal(rec.Body.Bytes(), &got); err != nil {
			t.Fatalf("body %q isn't JSON: %v", rec.Body, err)
		}
		created := clock.Now().Format(time.RFC3339)
		if got.Code != code || got.URL != "https://go.dev" || got.Hits != 1 || got.Created != created {
			t.Fatalf("stats %+v, want code %q, url https://go.dev, 1 hit, created %s", got, code, created)
		}
	})
	t.Run("c20_stats_for_an_unknown_code_are_a_404", func(t *testing.T) {
		defer failOnPanic(t)
		rec := send(NewServer(NewStore(newClock().Now)), "GET", "/links/nope", "")
		if rec.Code != http.StatusNotFound {
			t.Fatalf("status %d, want 404", rec.Code)
		}
	})
}
