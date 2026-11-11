// Part of the URL shortener project (tracks/go/extras/shortener-concurrency).
// Don't edit this file: `learn` overwrites it. Your code goes in store.go.
//
// A map that two goroutines write at once can crash the whole program with
// "fatal error: concurrent map writes", so a store that isn't safe yet may
// stop these tests early rather than fail one case.

package shortener

import (
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"testing"
)

func TestConcurrency(t *testing.T) {
	t.Run("c28_parallel_adds_get_unique_codes", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		var mu sync.Mutex
		codes := map[string]bool{}
		var wg sync.WaitGroup
		for g := 0; g < 50; g++ {
			wg.Add(1)
			go func() {
				defer wg.Done()
				defer func() { recover() }() // a TODO panic fails the count below instead
				for i := 0; i < 20; i++ {
					code, err := s.Add("https://go.dev")
					if err == nil {
						mu.Lock()
						codes[code] = true
						mu.Unlock()
					}
				}
			}()
		}
		wg.Wait()
		if len(codes) != 1000 {
			t.Fatalf("1000 adds gave %d different codes, want 1000", len(codes))
		}
	})
	t.Run("c29_parallel_visits_count_every_hit", func(t *testing.T) {
		defer failOnPanic(t)
		s := NewStore(newClock().Now)
		code, _ := s.Add("https://go.dev")
		var wg sync.WaitGroup
		for g := 0; g < 100; g++ {
			wg.Add(1)
			go func() {
				defer wg.Done()
				defer func() { recover() }()
				for i := 0; i < 10; i++ {
					s.Visit(code)
				}
			}()
		}
		wg.Wait()
		if got, _ := s.Get(code); got.Hits != 1000 {
			t.Fatalf("1000 visits counted %d hits, want 1000", got.Hits)
		}
	})
	t.Run("c30_parallel_requests_all_succeed", func(t *testing.T) {
		defer failOnPanic(t)
		h := NewServer(NewStore(newClock().Now))
		statuses := make(chan string, 100)
		var wg sync.WaitGroup
		for g := 0; g < 50; g++ {
			wg.Add(1)
			go func() {
				defer wg.Done()
				defer func() {
					if r := recover(); r != nil {
						statuses <- fmt.Sprint("panic: ", r)
					}
				}()
				created := send(h, "POST", "/links", `{"url": "https://go.dev"}`)
				var body struct{ Code string }
				json.Unmarshal(created.Body.Bytes(), &body)
				redirect := send(h, "GET", "/"+body.Code, "")
				statuses <- fmt.Sprint(created.Code, " ", redirect.Code)
			}()
		}
		wg.Wait()
		close(statuses)
		for got := range statuses {
			if got != fmt.Sprint(http.StatusCreated, " ", http.StatusFound) {
				t.Fatalf("a create and redirect gave %q, want \"201 302\"", got)
			}
		}
	})
}
