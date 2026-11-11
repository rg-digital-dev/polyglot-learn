// Part of the URL shortener project (tracks/go/extras/shortener-store). Don't
// edit this file: `learn` overwrites it. Your code goes in store.go.
//
// The helpers at the bottom (a clock you can move, a panic guard) are used by
// every stage's tests.

package shortener

import (
	"errors"
	"sync"
	"testing"
	"time"
)

func TestStore(t *testing.T) {
	t.Run("c01_the_first_link_gets_code_1", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, err := s.Add("https://go.dev")
		if err != nil || code != "1" {
			t.Fatalf("Add = %q, %v; want \"1\", nil", code, err)
		}
	})
	t.Run("c02_codes_count_up_in_base_62", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		var codes []string
		for i := 0; i < 62; i++ {
			code, err := s.Add("https://go.dev")
			if err != nil {
				t.Fatalf("Add: %v", err)
			}
			codes = append(codes, code)
		}
		for n, want := range map[int]string{1: "1", 9: "9", 10: "a", 35: "z", 36: "A", 61: "Z", 62: "10"} {
			if codes[n-1] != want {
				t.Errorf("link %d got code %q, want %q", n, codes[n-1], want)
			}
		}
	})
	t.Run("c03_get_returns_the_stored_link", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		code, _ := s.Add("https://go.dev/doc")
		got, err := s.Get(code)
		want := Link{Code: code, URL: "https://go.dev/doc", Hits: 0, Created: clock.Now()}
		if err != nil || got != want {
			t.Fatalf("Get(%q) = %+v, %v; want %+v, nil", code, got, err, want)
		}
	})
	t.Run("c04_an_unknown_code_is_errnotfound", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		if _, err := s.Get("nope"); !errors.Is(err, ErrNotFound) {
			t.Fatalf("Get(\"nope\") error = %v, want ErrNotFound", err)
		}
	})
	t.Run("c05_only_http_and_https_urls_are_accepted", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		for _, u := range []string{"ftp://files.example.com", "mailto:ada@example.com", "javascript:alert(1)"} {
			if _, err := s.Add(u); !errors.Is(err, ErrInvalidURL) {
				t.Errorf("Add(%q) error = %v, want ErrInvalidURL", u, err)
			}
		}
		if _, err := s.Add("http://example.com"); err != nil {
			t.Errorf("Add(\"http://example.com\") error = %v, want nil", err)
		}
	})
	t.Run("c06_a_url_needs_a_host", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		for _, u := range []string{"http://", "https:///just/a/path", "not a url", ""} {
			if _, err := s.Add(u); !errors.Is(err, ErrInvalidURL) {
				t.Errorf("Add(%q) error = %v, want ErrInvalidURL", u, err)
			}
		}
	})
	t.Run("c07_spaces_around_the_url_are_trimmed", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, err := s.Add("  https://go.dev \n")
		if err != nil {
			t.Fatalf("Add: %v", err)
		}
		if got, _ := s.Get(code); got.URL != "https://go.dev" {
			t.Fatalf("stored URL %q, want \"https://go.dev\"", got.URL)
		}
	})
	t.Run("c08_a_rejected_url_uses_no_code", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		s.Add("ftp://nope")
		if code, _ := s.Add("https://go.dev"); code != "1" {
			t.Fatalf("first accepted link got code %q, want \"1\"", code)
		}
	})
}

// clock is a time source the tests move by hand.
type clock struct {
	mu  sync.Mutex
	now time.Time
}

func newClock() *clock {
	return &clock{now: time.Date(2026, 3, 14, 15, 9, 26, 0, time.UTC)}
}

func (c *clock) Now() time.Time {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.now
}

func (c *clock) Advance(d time.Duration) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.now = c.now.Add(d)
}

// failOnPanic turns a panic (a TODO, say) into a failed case.
func failOnPanic(t *testing.T) {
	if r := recover(); r != nil {
		t.Fatalf("panicked: %v", r)
	}
}
