# Changing values in maps, and JSON out

A Go map holds values, and reading one gives you a copy. Changing the copy changes nothing in the map. To update a stored struct, either write it back or store pointers to structs, so the map and your variable share the one value.

```go
package main

import "fmt"

type Counter struct{ Hits int }

func main() {
	byValue := map[string]Counter{"a": {}}
	c := byValue["a"]
	c.Hits++                  // changes the copy only
	fmt.Println(byValue["a"]) // {0}
	byValue["a"] = c          // write it back
	fmt.Println(byValue["a"]) // {1}

	byPointer := map[string]*Counter{"a": {}}
	byPointer["a"].Hits++     // changes the stored struct
	fmt.Println(*byPointer["a"]) // {1}
}
```

`byValue["a"].Hits++` doesn't even compile ("cannot assign to struct field in map"), which is Go reminding you of the copy.

For JSON output, a `map[string]any` is quick, and a struct with tags gives a fixed shape. Times have no JSON type; send them as text, and `time.RFC3339` (`2026-03-14T15:09:26Z`) is the format most APIs use.

<!-- from:typescript -->
## Coming from TypeScript
- In TypeScript, objects in a `Map` are references, so `map.get(k).hits++` works. In Go that's only true for pointers (`*Counter`); a plain struct value behaves like a copied primitive.
- `time.Time` doesn't turn into a string by itself the way a `Date` turns into ISO text in `JSON.stringify`; Go's encoder does write RFC 3339 for a `time.Time` field, but formatting it yourself keeps the output exact.
<!-- /from -->

<!-- from:default -->
## Key points
- Reading a struct from a map copies it; update by writing back, or keep pointers in the map.
- A method that returns a `Link` value hands the caller a copy, so callers can't change the store behind its back.
- Two routes can overlap: `GET /links/{code}` and `GET /{code}` both match `/links/x`, and the mux picks the more specific one.
<!-- /from -->

## Common mistake
Counting hits on the copy you returned: `link, _ := s.Get(code); link.Hits++` looks right and passes until a test reads the hits back from the store.

## In this stage
Implement `Visit` in `stats.go`: like `Get`, but it adds one to the stored link's hits and returns the updated link. Then change `server.go`: the redirect uses `Visit`, and `GET /links/{code}` answers `{"code", "url", "hits", "created"}` with `created` in RFC 3339, or a 404. `Get` itself must not count.
