# Text in TypeScript

JavaScript strings are UTF-16. `s.length`, `s[i]`, `slice` and `substring` all work in UTF-16 code units, and an emoji outside the Basic Multilingual Plane takes two of them (a surrogate pair). The string *iterator* works in code points instead: `for (const ch of s)`, `[...s]` and `Array.from(s)` give one string per code point. `codePointAt` reads a whole code point; `charCodeAt` reads one unit.

```ts
const s = "Zoë 😀";
console.log(s.length);              // 6 UTF-16 units
console.log([...s].length);         // 5 code points
console.log(new TextEncoder().encode(s).length);   // 9 UTF-8 bytes
console.log(s.slice(-1));           // "\ude00": half an emoji
console.log([...s].slice(-1)[0]);   // "😀"
```

<!-- from:javascript -->
## Coming from JavaScript
- Exactly the same strings: TypeScript adds no string type of its own, and `string` doesn't know about code points.
- With `noUncheckedIndexedAccess`, `[...s][i]` is `string | undefined`, which is a reminder the index may be past the end.
<!-- /from -->

<!-- from:default -->
## Key points
- `length`, indexing and `slice` count UTF-16 units.
- `for...of`, spread and `Array.from` go by code point.
- `Intl.Segmenter` (with `granularity: "grapheme"`) goes by what a reader sees.
<!-- /from -->

## Common mistake
Slicing an emoji in half:

```ts
const face = "😀!";
face.slice(0, 1);              // "\ud83d": a lone surrogate, shows as �
[...face].slice(0, 1).join("") // "😀"
```

## In these exercises
`charCount(text)` maps each code point to its count; loop with `for...of`. `truncate(text, limit)` must measure and cut in code points too: spread the string into an array first.
