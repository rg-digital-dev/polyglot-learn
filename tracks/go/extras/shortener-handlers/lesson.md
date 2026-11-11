# HTTP handlers and JSON

`net/http` is a production web server in the standard library. A handler is anything with `ServeHTTP(w http.ResponseWriter, r *http.Request)`; `http.HandlerFunc` turns a plain function into one. Since Go 1.22, `http.ServeMux` routes on method and path with wildcards, so most services need no router package.

```go
package main

import (
	"encoding/json"
	"net/http"
)

type greeting struct {
	Name string `json:"name"` // the tag names the JSON field
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json") // headers first...
	w.WriteHeader(status)                              // ...then the status...
	json.NewEncoder(w).Encode(v)                       // ...then the body
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("POST /greetings", func(w http.ResponseWriter, r *http.Request) {
		var g greeting
		if err := json.NewDecoder(r.Body).Decode(&g); err != nil {
			writeJSON(w, http.StatusBadRequest, map[string]string{"error": "invalid JSON"})
			return
		}
		writeJSON(w, http.StatusCreated, map[string]string{"hello": g.Name})
	})
	mux.HandleFunc("GET /hello/{name}", func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]string{"hello": r.PathValue("name")})
	})
	http.ListenAndServe("localhost:8080", mux)
}
```

Tests don't need a network: `httptest.NewRecorder()` is a ResponseWriter that records what was written, and `httptest.NewRequest` builds a request, so `h.ServeHTTP(rec, req)` runs a handler in memory.

<!-- from:typescript -->
## Coming from TypeScript
- A handler is Express's `(req, res) => …`, but the response is written in three steps: headers, then `WriteHeader(status)`, then the body. There's no `res.json()`; a three-line helper does the same.
- Struct tags (`json:"name"`) map Go's capitalised field names to JSON's; only exported (capitalised) fields are encoded at all.
- `mux.HandleFunc("GET /hello/{name}", …)` is Express's `app.get("/hello/:name", …)`, and `r.PathValue("name")` is `req.params.name`.
<!-- /from -->

<!-- from:default -->
## Key points
- Order matters: headers, then `WriteHeader`, then the body. The first `Write` sends a 200 if you didn't set a status.
- `json.NewDecoder(r.Body).Decode(&v)` reads and checks the body in one go; a decoding error is the client's fault, a 400.
- Return right after writing an error; the rest of the handler mustn't write a second response.
<!-- /from -->

## Common mistake
Setting a header after `WriteHeader` (or after the first `Write`) does nothing, silently: the headers are already on their way. If a response arrives as `text/plain` instead of JSON, look for a `Write` that happened before `Header().Set`.

## In this stage
Implement `NewServer` in `server.go`: `POST /links` with `{"url": "..."}` stores the link and answers 201 with `{"code": "...", "url": "..."}`; `GET /{code}` redirects (302) to the link's URL. Every JSON response has `Content-Type: application/json`, and errors are `{"error": "..."}`: 400 for a body that isn't JSON or a URL the store rejects (with the store's error message, "invalid URL"), 404 for a code it doesn't know.
