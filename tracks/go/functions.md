# Functions and strings in Go

A Go function is declared with `func`, parameter names come before their types, and the return type follows the parameter list: `func Name(x T) R`. Functions can return several values, and a name that starts with a capital letter is exported from its package. A Go `string` is an immutable sequence of bytes, usually holding UTF-8 text. The `strings` package has the common helpers (trimming, splitting, case changes), and `fmt.Sprintf` builds a new string from a format with verbs like `%s` and `%d`.

```go
package shop

import (
	"fmt"
	"strings"
)

// Label is exported because it starts with a capital letter.
func Label(item string, qty int) string {
	item = strings.ToLower(strings.TrimSpace(item))
	if qty == 1 {
		return fmt.Sprintf("1 %s", item)
	}
	return fmt.Sprintf("%d %ss", qty, item)
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Types come after names: `func Label(item string, qty int) string`, not `(item: string): string`.
- There are no template literals; use `fmt.Sprintf` or `+` concatenation.
- Every function needs an explicit `return`; there are no implicit return values or arrow-function shortcuts.
- Capitalization is visibility: `Label` is public, `label` is private to the package.
- `len(s)` counts bytes, not characters, and `s[0]` is a byte, not a one-letter string.
<!-- /from -->

<!-- from:default -->
## Key points
- Parameter types follow names; consecutive parameters of one type can share it: `func f(a, b string)`.
- Strings are immutable; every helper returns a new string.
- `strings.TrimSpace`, `strings.ToLower` and `fmt.Sprintf` cover most everyday text work.
<!-- /from -->

## Common mistake
Using `len` to count characters:

```go
len("Zoë")                     // 4: the ë takes two bytes
utf8.RuneCountInString("Zoë")  // 3: import "unicode/utf8"
```

## In this exercise
Implement `func Greet(name string) string`. Return the full sentence, e.g. `"Hello, Alice!"`, with surrounding whitespace removed and `"world"` used when nothing is left.
