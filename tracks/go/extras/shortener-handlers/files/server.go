package shortener

import "net/http"

// NewServer routes the shortener's HTTP API to store:
//
//	POST /links   {"url": "..."}  ->  201 {"code": "...", "url": "..."}
//	GET  /{code}                  ->  302 to the link's URL, or 404
//
// Every JSON response has Content-Type application/json, and errors are
// {"error": "..."}: a 400 for a body that isn't JSON or a URL the store
// rejects (with the store's error message), a 404 for an unknown code.
func NewServer(store *Store) http.Handler {
	panic("TODO: implement NewServer")
}
