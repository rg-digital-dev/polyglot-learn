# Maps in QML

For string-keyed data, QML code uses plain JavaScript objects: `const m = {}`, `m[key] = value`, `key in m`, `delete m[key]`, and `Object.keys(m)` / `Object.entries(m)` for iteration. A missing key reads as `undefined`, so a counter is usually bumped with `(m[key] ?? 0) + 1`. Objects cross into and out of QML functions as `var`. The same pattern appears in Quickshell configs that group windows by workspace or count notifications per app before showing them in a `Repeater`.

```qml
import QtQuick

QtObject {
    // Count file extensions in a list of paths.
    function countExtensions(paths: var): var {
        const counts = {}
        for (const path of paths) {
            const dot = path.lastIndexOf(".")
            const ext = dot > 0 ? path.slice(dot + 1).toLowerCase() : "(none)"
            counts[ext] = (counts[ext] ?? 0) + 1
        }
        return counts
    }

    function total(counts: var): int {
        return Object.values(counts).reduce((sum, n) => sum + n, 0)
    }
}
// countExtensions(["a.QML", "b.txt", "c.qml", "Makefile"])
//   -> { qml: 2, txt: 1, "(none)": 1 }        total(...) -> 4
```

<!-- from:typescript -->
## Coming from TypeScript
- Return a plain object (`{}`), not a `Map`. The test's `compare()` checks an object's keys and values, and a `Map` never equals an object literal.
- There is no `Record<string, number>` type; the annotation is just `var`.
- `??` and `?.` work in Qt 6, so `(m[k] ?? 0) + 1` is fine.
- `Object.keys(m).length` gives the size; objects have no `.size`.
<!-- /from -->

<!-- from:default -->
## Key points
- Use a plain object as a string-keyed map and return it as `var`.
- `(m[k] ?? 0) + 1` increments a counter that may not exist yet.
- Iterate with `Object.keys`, `Object.values` or `Object.entries`.
- `toLowerCase()` returns a new string; strings are immutable.
<!-- /from -->

## Common mistake
Incrementing a key that does not exist yet:

```qml
counts[word]++                              // undefined + 1 -> NaN
counts[word] = (counts[word] ?? 0) + 1      // fixed
```

A plain `{}` also inherits keys from `Object.prototype`: the word `"constructor"` already "exists", so `(counts[w] ?? 0) + 1` gives a string of function source instead of `1`. Start from `Object.create(null)`, which has no inherited keys.

## In this exercise
Implement `function wordCount(text: string): var` in `word-count/WordCount.qml`. Return a plain object mapping each lowercase word to its count; empty text gives `{}`. Key order does not matter to `compare()`.
