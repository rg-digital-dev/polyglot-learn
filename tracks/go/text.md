# Text in Go

A Go `string` is a read-only slice of bytes, usually UTF-8. `len(s)` counts bytes, `s[i]` is a byte, and `s[a:b]` cuts at byte offsets, even in the middle of a character. A `rune` is one code point (an `int32`). `for i, r := range s` decodes the string one rune at a time (`i` is the byte offset), `utf8.RuneCountInString(s)` counts runes, and `[]rune(s)` converts the whole string to a slice of runes you can index and slice by code point.

```go
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	s := "Zoë 😀"
	fmt.Println(len(s), utf8.RuneCountInString(s)) // 9 5
	for i, r := range s {
		fmt.Printf("%d:%c ", i, r) // 0:Z 1:o 2:ë 4:  5:😀
	}
	fmt.Println(string([]rune(s)[:3])) // Zoë
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `len(s)` is the UTF-8 byte length, not UTF-16 units and not characters.
- `range s` is `for (const ch of s)`, but gives a `rune` (a number); `string(r)` makes it text.
- `[]rune(s)` is `[...s]`.
<!-- /from -->

<!-- from:default -->
## Key points
- `len`, indexing and slicing work in bytes.
- `range` over a string, `[]rune(s)` and `unicode/utf8` work in code points.
- `string(r)` turns one rune into a one-character string.
<!-- /from -->

## Common mistake
Slicing the string itself:

```go
s := "résumé"
fmt.Printf("%q\n", s[:2])                  // "r\xc3": half of é
fmt.Printf("%q\n", string([]rune(s)[:2]))  // "ré"
```

## In these exercises
`CharCount(text)` returns `map[string]int`; range over the string and use `string(r)` as the key. `Truncate(text, limit)` should count and cut in runes: `[]rune(text)` does both.
