# References in QML's JavaScript

Functions in QML run JavaScript, with JavaScript's rules: arrays and objects are shared by `=`, by arguments and by return values, and numbers, strings and booleans are copied. `a.slice()` copies an array one level deep; for nested arrays, copy each inner one too. Properties add a twist: assigning a JavaScript array to a typed property such as `property list<int>` converts it, so the property holds a copy, while `property var` keeps the reference.

```qml
import QtQuick

QtObject {
    Component.onCompleted: {
        const a = [[1, 2], [3]]
        const b = a                                // the same array
        const c = a.slice()                        // a new outer array, the same rows
        const d = a.map(row => row.slice())        // a deep copy

        a[0][0] = 9
        console.log(b[0], c[0], d[0])              // [9,2] [9,2] [1,2]
        console.log(b === a, c === a, c[0] === a[0])   // true false true
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- The same sharing rules as TypeScript.
- `a.slice()`, `[...a]` and `a.map(r => r.slice())` all work.
- Typed properties (`list<int>`, `point`, ...) copy on assignment; `var` properties share.
<!-- /from -->

<!-- from:default -->
## Key points
- Arrays and objects are shared; `slice()` copies one level.
- Copy nested arrays one by one for a deep copy.
- A typed list property holds a copy, a `var` property a reference.
<!-- /from -->

## Common mistake
`fill` repeats one array:

```js
const grid = new Array(2).fill(new Array(3).fill(0))   // two references to one row
grid[0][0] = 9                                         // both rows start with 9

const fixed = Array.from({ length: 2 }, () => new Array(3).fill(0))
```

## In these exercises
`makeGrid(rows, cols, fill)` builds a new array per row. `addBonus(scores, name, points)` returns a new object whose lists are copies (`slice()` each one), with `points` pushed onto `name`'s copy; don't change `scores`.
