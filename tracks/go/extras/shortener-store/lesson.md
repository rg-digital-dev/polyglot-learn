# Project: a URL shortener in Go

This project builds one real program over eight stages: a URL shortener, the kind of small HTTP service Go is made for. It starts as a plain Go type and ends as a server that runs many requests at once, times out slow ones, expires old links and shuts down cleanly. Every stage keeps the code you wrote before and adds tests, so the earlier stages keep checking that nothing broke. Only the standard library is used.

The first stage is the store: a type that keeps links under short codes. Two things in it are worth getting right from the start: values whose zero state isn't usable get a constructor, and errors that callers need to tell apart are package-level *sentinel* values.

```go
package library

import (
	"errors"
	"time"
)

// Sentinel errors: callers compare with errors.Is.
var ErrNoSuchBook = errors.New("no such book")

type Book struct {
	ID    int
	Title string
	Added time.Time
}

type Library struct {
	books map[int]Book
	next  int
	now   func() time.Time // the clock, passed in so tests can control it
}

func NewLibrary(now func() time.Time) *Library {
	return &Library{books: map[int]Book{}, now: now} // a nil map would panic on write
}

func (l *Library) Add(title string) int {
	l.next++
	l.books[l.next] = Book{ID: l.next, Title: title, Added: l.now()}
	return l.next
}

func (l *Library) Get(id int) (Book, error) {
	b, ok := l.books[id]
	if !ok {
		return Book{}, ErrNoSuchBook
	}
	return b, nil
}
```

`NewLibrary(time.Now)` uses the real clock; a test passes a function that returns whatever time it likes.

<!-- from:typescript -->
## Coming from TypeScript
- Errors are values returned next to the result, not thrown. `ErrNoSuchBook` plays the role of an error subclass; `errors.Is(err, ErrNoSuchBook)` is the `instanceof` check, and it also sees through wrapped errors.
- A struct with methods is Go's class. `NewLibrary` is a convention, not a constructor keyword: it's just a function that returns a ready value.
- Lower-case fields (`books`, `next`) are private to the package, like `#private` fields.
<!-- /from -->

<!-- from:default -->
## Key points
- Make the map in the constructor: reading a nil map works, writing to one panics.
- Return sentinel errors as they are, and compare them with `errors.Is`, not `==` on the message.
- Passing the clock in (`now func() time.Time`) keeps time-dependent code testable without sleeping.
<!-- /from -->

## Common mistake
`url.Parse` hardly ever fails: `url.Parse("not a url")` returns a URL with only a path, and no error. Checking `err == nil` isn't validation; check the parts you need, such as the scheme and the host.

## In this stage
Implement `NewStore`, `Add` and `Get` in `store.go`. Codes are the link's number in base 62, using the digits `0-9`, then `a-z`, then `A-Z`: the first link is `1`, the 10th is `a`, the 62nd is `10`. A URL is accepted when, after trimming spaces, it parses with the scheme `http` or `https` and a host; anything else is `ErrInvalidURL` and uses up no code.
