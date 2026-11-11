package shortener

import "time"

// AddTTL is Add for a link that expires ttl after it's added, by the store's
// clock. From that moment on, Get and Visit treat it as gone (ErrNotFound),
// even before Expire has removed it: change them in store.go and stats.go.
// Links from Add never expire.
func (s *Store) AddTTL(rawURL string, ttl time.Duration) (string, error) {
	panic("TODO: implement AddTTL")
}

// Expire removes every expired link and returns how many it removed.
func (s *Store) Expire() int {
	panic("TODO: implement Expire")
}

// StartJanitor starts a goroutine that calls Expire every interval (use a
// time.Ticker), until the returned stop function is called. Calling stop more
// than once must be safe.
func (s *Store) StartJanitor(every time.Duration) (stop func()) {
	panic("TODO: implement StartJanitor")
}
