# Sharing state safely: sync.Mutex

`net/http` serves every request in its own goroutine, so a server is concurrent whether you planned it or not: two requests can be inside your store at the same moment. Go maps aren't safe for that. Two goroutines writing one map at once can crash the program with `fatal error: concurrent map writes`, and a counter updated as read-add-write loses updates when two goroutines interleave.

A `sync.Mutex` lets one goroutine at a time into a section of code. `sync.RWMutex` adds a shared mode: any number of readers at once (`RLock`), or one writer alone (`Lock`).

```go
package main

import (
	"fmt"
	"sync"
)

type Scores struct {
	mu     sync.RWMutex // guards points
	points map[string]int
}

func (s *Scores) Add(name string, n int) {
	s.mu.Lock()
	defer s.mu.Unlock() // released however the method returns
	s.points[name] += n
}

func (s *Scores) Get(name string) int {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.points[name]
}

func main() {
	s := &Scores{points: map[string]int{}}
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			s.Add("ada", 1)
		}()
	}
	wg.Wait()
	fmt.Println(s.Get("ada")) // always 100
}
```

Go's race detector finds unguarded access while the program runs: `go test -race`. It needs cgo (a C compiler), so it isn't part of these tests, but it's worth running on your own machine if you have one.

<!-- from:typescript -->
## Coming from TypeScript
- JavaScript runs your code on one thread, so `count++` can't be interrupted. Goroutines really do run at the same time, on several cores, and `count++` is three steps another goroutine can slip between.
- A mutex is like a lock around an `await`-free critical section that you have to take explicitly; there's no runtime doing it for you.
<!-- /from -->

<!-- from:default -->
## Key points
- Put the mutex in the struct next to what it guards, and say so in a comment.
- `Lock` + `defer Unlock` at the top of each method is the simple, safe default.
- Use `RLock` only where nothing is changed: a method that updates counts needs `Lock`.
- A struct holding a mutex must not be copied after use; pass `*Store`, never `Store`.
<!-- /from -->

## Common mistake
Taking the lock in one method and calling another method that takes it too: Go's mutexes aren't reentrant, so `Add` calling a locked `Get` waits for itself forever. Keep the locking in the public methods and give them unlocked helpers to share.

## In this stage
Make the store in `store.go` (and `Visit` in `stats.go`) safe for many goroutines at once. The tests add 1000 links from 50 goroutines and expect 1000 different codes, count 1000 visits from 100 goroutines, and send 50 requests in parallel through the server.
