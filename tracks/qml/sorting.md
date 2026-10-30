# Collections and sorting in QML

Lists in QML functions are JavaScript arrays. `push`, `slice`, `map`, `filter`, `includes` and friends all work, and `sort(compareFn)` orders an array in place and returns the same array. The comparator returns a negative number when `a` comes first, a positive one when `b` does, and `0` for a tie. Without a comparator, `sort()` compares elements as strings. QML also has typed sequence properties such as `list<int>`, which behave mostly like arrays; data passed through a `var` parameter, as in this course, is a plain array. In Quickshell, sorted arrays like these often become the `model` of a `Repeater`, for example to list workspaces in order.

```qml
import QtQuick

QtObject {
    // Highest level first; equal levels by name. The input is left untouched.
    function ranked(players: var): var {
        return players.slice().sort((a, b) =>
            b.level - a.level || a.name.localeCompare(b.name))
    }

    function names(players: var): var {
        return players.map(p => p.name)
    }
}
// const players = [{ name: "kim", level: 7 }, { name: "ava", level: 9 }, { name: "lou", level: 7 }]
// names(ranked(players)) -> ["ava", "kim", "lou"]     players[0].name -> "kim"
```

<!-- from:typescript -->
## Coming from TypeScript
- `sort` mutates and returns the same array, exactly as in TypeScript. Copy first with `slice()` or `[...arr]`.
- The default `sort()` is lexicographic here too: `[10, 9, 100].sort()` gives `[10, 100, 9]`. Always pass `(a, b) => b - a` for numbers.
- Newer helpers such as `toSorted()` are beyond the ECMAScript 7 baseline that the Qt docs promise, so stick to `slice().sort()`.
- `slice(0, n)` already stops at the end of the array; no bounds check needed.
<!-- /from -->

<!-- from:default -->
## Key points
- `sort(compareFn)` sorts in place; copy with `slice()` to keep the input unchanged.
- For numbers, descending order is `(a, b) => b - a`.
- `slice(0, n)` takes at most the first `n` items.
- `compare()` in the tests checks arrays element by element.
<!-- /from -->

## Common mistake
Sorting numbers without a comparator:

```qml
[5, 40, 100].sort()                  // [100, 40, 5]: "100" < "40" < "5" as strings
[5, 40, 100].sort((a, b) => a - b)   // fixed: [5, 40, 100]
```

## In this exercise
Implement `function topScores(scores: var, n: int): var` in `top-scores/TopScores.qml`. Return a new array with at most `n` scores, highest first, and leave `scores` unchanged.
