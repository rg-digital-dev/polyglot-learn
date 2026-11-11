package shortener

import (
	"errors"
	"time"
)

var (
	// ErrInvalidURL is returned by Add for anything but an http or https URL with a host.
	ErrInvalidURL = errors.New("invalid URL")
	// ErrNotFound is returned for a code the store doesn't have.
	ErrNotFound = errors.New("not found")
)

// Link is one short link.
type Link struct {
	Code    string
	URL     string
	Hits    int
	Created time.Time
}

// Store keeps the links. Give it the fields it needs: the links by code, the
// number for the next code, and the clock.
type Store struct {
}

// NewStore returns an empty store that reads the time from now.
func NewStore(now func() time.Time) *Store {
	panic("TODO: implement NewStore")
}

// Add checks rawURL (spaces around it trimmed): it must parse, use http or
// https, and have a host, or Add returns ErrInvalidURL. A valid URL is stored
// under the next code in base 62 ("1", "2", … "9", "a", … "z", "A", … "Z",
// "10", …) with Created set from the clock, and Add returns the code.
func (s *Store) Add(rawURL string) (string, error) {
	panic("TODO: implement Add")
}

// Get returns the link stored under code, or ErrNotFound.
func (s *Store) Get(code string) (Link, error) {
	panic("TODO: implement Get")
}
