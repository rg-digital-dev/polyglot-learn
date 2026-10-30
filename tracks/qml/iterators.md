# Pipelines in QML

QML's JavaScript has the usual array methods, and the idiomatic way to process a list is a chain of them: `filter` to keep items, `map` to transform them, `sort` to order them, and `slice(0, n)` to take the first few. `filter`, `map` and `slice` return new arrays; `sort` reorders the array in place, which is harmless when an earlier step already made a copy. To run a pipeline over a map, start with `Object.entries(obj)`, which gives `[key, value]` pairs. Iteration with `for...of` and generator functions (`function*`) work too. Some newer helpers do not: Qt 6.10's engine has no `flatMap`, `Object.fromEntries`, `toSorted`, `at` or `matchAll`.

```qml
import QtQuick

QtObject {
    // files: [{ name: "Bar.qml", kb: 12 }, ...]; the n largest QML files, biggest first.
    function largestQml(files: var, n: int): var {
        return files
            .filter(f => f.name.endsWith(".qml"))
            .sort((a, b) => b.kb - a.kb || (a.name < b.name ? -1 : 1))
            .slice(0, n)
            .map(f => `${f.name} (${f.kb} KB)`)
    }

    // Object.entries turns a map into [key, value] pairs you can chain on.
    function busiest(hits: var): string {
        const ranked = Object.entries(hits).sort((a, b) => b[1] - a[1])
        return ranked.length > 0 ? ranked[0][0] : ""
    }
}
// largestQml([{ name: "Bar.qml", kb: 12 }, { name: "a.png", kb: 90 },
//             { name: "Shell.qml", kb: 30 }, { name: "Aux.qml", kb: 12 }], 2)
//   -> ["Shell.qml (30 KB)", "Aux.qml (12 KB)"]
// busiest({ "/": 3, "/docs": 9, "/blog": 1 }) -> "/docs"
```

<!-- from:typescript -->
## Coming from TypeScript
- `filter`, `map`, `slice`, `sort`, `Object.entries` and `for...of` behave as in TypeScript.
- Check for missing ES2019+ helpers. `flatMap`, `Object.fromEntries` and `toSorted` are undefined in Qt 6.10.
- `str.match(/re/g)` returns `null`, not `[]`, when nothing matches; add `|| []` before chaining.
- `a || b` in a comparator is the usual way to add a second sort key: it falls through when the first difference is `0`.
<!-- /from -->

<!-- from:default -->
## Key points
- Chain `filter`, `map`, `sort` and `slice(0, n)` instead of writing index loops.
- `Object.entries(obj)` turns a map into an array of `[key, value]` pairs.
- A comparator returns a negative number, zero or a positive number.
- `sort` changes the array it is called on; copy first if the input must stay intact.
<!-- /from -->

## Common mistake
Returning a boolean from the comparator. `true` and `false` become `1` and `0`, never negative, so the result is not sorted:

```qml
[3, 1, 4, 1, 5].sort((a, b) => a < b)      // some order, not a sorted one
[3, 1, 4, 1, 5].sort((a, b) => b - a)      // fixed: [5, 4, 3, 1, 1]
```

## In this exercise
Implement `function topWords(text: string, n: int): var` in `top-words/TopWords.qml`. Return an array of at most `n` lowercase words, most frequent first, with ties in alphabetical order. Every word appears once in the counts, so a comparator that checks count and then word fully decides the order.
