# Text in QML's JavaScript

QML strings are JavaScript strings: UTF-16. `s.length`, `s[i]` and `slice` count UTF-16 code units, so an emoji outside the Basic Multilingual Plane counts as two. `for (const ch of s)` and the spread `[...s]` go by code point. Two things differ from a browser or Node: QML has no `Intl` object at all (so no `Intl.Segmenter`), and in Qt 6's engine `Array.from(s)` still splits by UTF-16 unit. Use spread or `for...of`.

```qml
import QtQuick

QtObject {
    Component.onCompleted: {
        const s = "a😀"
        console.log(s.length)            // 3 UTF-16 units
        console.log([...s].length)       // 2 code points
        console.log(Array.from(s).length) // 3 in QML (2 in a browser)
        console.log(s.codePointAt(1))    // 128512, U+1F600
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- The same UTF-16 strings and the same code-point iterator.
- `Array.from(s)` is not a safe way to split by code point here; spread is.
- No `Intl.Segmenter`, and no `TextEncoder` either.
<!-- /from -->

<!-- from:default -->
## Key points
- `length`, indexing and `slice` count UTF-16 units.
- `for...of` and `[...s]` give one string per code point.
- A `string` property holds the same text; the rules above apply to it unchanged.
<!-- /from -->

## Common mistake
Cutting with `slice` can leave half an emoji:

```js
const face = "😀!"
face.slice(0, 1)               // "\ud83d", shows as �
[...face].slice(0, 1).join("") // "😀"
```

## In these exercises
`charCount(text)` maps each code point to its count; loop with `for...of`. `truncate(text, limit)` measures and cuts in code points: spread the string into an array first.
