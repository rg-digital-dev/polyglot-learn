package shortener

// Visit is Get that also counts a hit: it adds one to the stored link's Hits
// and returns the link as it is after that. An unknown code is ErrNotFound.
//
// Then, in server.go: make the redirect use Visit, and add
//
//	GET /links/{code}  ->  200 {"code", "url", "hits", "created"}, or 404
//
// with "created" as RFC 3339 text (time.RFC3339).
func (s *Store) Visit(code string) (Link, error) {
	panic("TODO: implement Visit")
}
