# Packages in Go

Go code is organised in packages, and a package is simply a directory: every `.go` file in it starts with the same `package` clause. A module, defined by `go.mod`, is a tree of packages with a common import path prefix. To use another package you import its full path, the module path followed by the directory, such as `"learn/slugify/words"`, and then call its contents through the package name: `words.Split(...)`. Visibility follows the first letter of a name: `Split` is exported, `split` is private to its package.

```go
// file units/units.go
package units

// ToKiB is exported: it starts with a capital letter.
func ToKiB(bytes int) int {
	return roundUp(bytes, 1024)
}

// roundUp is unexported: only code in package units can call it.
func roundUp(value, size int) int {
	return (value + size - 1) / size
}

// file report.go, one directory up, in a module named "learn"
package report

import (
	"fmt"

	"learn/report/units"
)

func Describe(bytes int) string {
	return fmt.Sprintf("%d KiB", units.ToKiB(bytes))
}
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `export` keyword; a capital first letter exports a name.
- Imports name a package, not a file, and always use the full path from the module root.
- Unlike `import { split } from "./words"`, you keep the package prefix at every call: `words.Split`.
- All files in one directory share one namespace; they can call each other's private functions.
<!-- /from -->

<!-- from:default -->
## Key points
- One directory is one package; the import path is the module path plus the directory.
- Exported names start with a capital letter; a lowercase name reached from outside is reported as undefined.
- Unused imports are a compile error, so add the import only once you call it.
<!-- /from -->

## Common mistake
Importing by relative path:

```go
import "./words"              // "./words" is relative, but relative import paths are not supported in module mode
import "learn/slugify/words"  // fixed: module path + directory
```

## In this exercise
Implement `func Split(text string) []string` in `slugify/words/words.go` (package `words`; its tests run first), then `func Slugify(title string) string` in `slugify/slugify.go`, which imports `"learn/slugify/words"` and calls `words.Split`.
