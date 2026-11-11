// Part of the URL shortener project (tracks/go/extras/shortener-expiry).
// Don't edit this file: `learn` overwrites it. Your code goes in expiry.go and
// store.go.
//
// Expiry follows the store's clock, which these tests move by hand; the
// janitor runs on real time, every 10 ms.

package shortener

import (
	"errors"
	"testing"
	"time"
)

func TestExpiry(t *testing.T) {
	t.Run("c35_a_link_without_a_ttl_never_expires", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		code, _ := s.Add("https://go.dev")
		clock.Advance(10 * 365 * 24 * time.Hour)
		if s.Expire() != 0 {
			t.Fatal("Expire removed a link without a TTL")
		}
		if _, err := s.Get(code); err != nil {
			t.Fatalf("Get after ten years: %v", err)
		}
	})
	t.Run("c36_an_expired_link_is_not_found", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		code, err := s.AddTTL("https://go.dev", time.Hour)
		if err != nil {
			t.Fatalf("AddTTL: %v", err)
		}
		clock.Advance(59 * time.Minute)
		if _, err := s.Get(code); err != nil {
			t.Fatalf("Get a minute before expiry: %v", err)
		}
		clock.Advance(time.Minute)
		if _, err := s.Get(code); !errors.Is(err, ErrNotFound) {
			t.Fatalf("Get at expiry: error %v, want ErrNotFound", err)
		}
		if _, err := s.Visit(code); !errors.Is(err, ErrNotFound) {
			t.Fatalf("Visit at expiry: error %v, want ErrNotFound", err)
		}
	})
	t.Run("c37_expire_removes_expired_links_and_counts_them", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		s.AddTTL("https://go.dev/a", time.Minute)
		s.AddTTL("https://go.dev/b", time.Hour)
		keep, _ := s.AddTTL("https://go.dev/c", 3*time.Hour)
		clock.Advance(2 * time.Hour)
		if n := s.Expire(); n != 2 {
			t.Fatalf("Expire removed %d links, want 2", n)
		}
		if n := s.Expire(); n != 0 {
			t.Fatalf("a second Expire removed %d, want 0", n)
		}
		if _, err := s.Get(keep); err != nil {
			t.Fatalf("the link with time left is gone: %v", err)
		}
	})
	t.Run("c38_the_janitor_expires_links_in_the_background", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		stop := s.StartJanitor(10 * time.Millisecond)
		defer stop()
		s.AddTTL("https://go.dev", time.Minute)
		clock.Advance(time.Hour)
		time.Sleep(300 * time.Millisecond)
		if n := s.Expire(); n != 0 {
			t.Fatalf("after 300ms the janitor still hadn't removed the expired link (Expire found %d)", n)
		}
	})
	t.Run("c39_stop_ends_the_janitor_and_is_safe_twice", func(t *testing.T) {
		defer failOnPanic(t)
		clock := newClock()
		s := NewStore(clock.Now)
		stop := s.StartJanitor(10 * time.Millisecond)
		stop()
		stop()
		time.Sleep(50 * time.Millisecond) // let a tick that was already due pass
		s.AddTTL("https://go.dev", time.Minute)
		clock.Advance(time.Hour)
		time.Sleep(200 * time.Millisecond)
		if n := s.Expire(); n != 1 {
			t.Fatalf("Expire found %d expired links, want 1: the janitor kept running after stop", n)
		}
	})
}
