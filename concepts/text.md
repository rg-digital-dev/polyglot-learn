# Text and Unicode

"How long is this string?" has more than one answer. Text is stored as bytes in some encoding, and every language picks one of these units for its basic `length` and indexing:

- **Bytes.** UTF-8 uses 1 byte for ASCII, 2 for `é`, 3 for `日` and 4 for an emoji.
- **UTF-16 code units.** JavaScript strings are UTF-16: most characters take one unit, but an emoji outside the Basic Multilingual Plane takes two (a *surrogate pair*).
- **Code points.** One number per Unicode character: `é` is U+00E9, `😀` is U+1F600. This is the unit the exercises here use.
- **Graphemes.** What a reader sees as one character. That can be several code points: `e` followed by a combining accent U+0301 shows as `é`, and a family emoji is several emoji joined by U+200D.

| Track | Basic length counts | Loop over code points | Count code points | First `n` code points |
|---|---|---|---|---|
| TypeScript | UTF-16 units (`s.length`) | `for (const ch of s)` | `[...s].length` | `[...s].slice(0, n).join("")` |
| Python | code points (`len(s)`) | `for ch in s` | `len(s)` | `s[:n]` |
| Rust | bytes (`s.len()`) | `for ch in s.chars()` | `s.chars().count()` | `s.chars().take(n).collect::<String>()` |
| Go | bytes (`len(s)`) | `for _, r := range s` | `utf8.RuneCountInString(s)` | `string([]rune(s)[:n])` |
| PHP | bytes (`strlen`) | `preg_split('//u', $s, -1, PREG_SPLIT_NO_EMPTY)` | `count(...)` of that, or `mb_strlen` | `iconv_substr($s, 0, $n)` or `mb_substr` |
| Lua | bytes (`#s`) | `for _, cp in utf8.codes(s)` | `utf8.len(s)` | `s:sub(1, utf8.offset(s, n + 1) - 1)` |
| QML | UTF-16 units (`s.length`) | `for (const ch of s)` | `[...s].length` | `[...s].slice(0, n).join("")` |

Python is the only track whose plain `len` and slices already count code points. Everywhere else, the obvious tool counts something smaller.

## Common mistake

Cutting by the wrong unit splits a character:

```text
"résumé" cut to 2 bytes   → "r" + half of "é"     (Go, PHP, Lua show broken text; Rust panics)
"😀😀"   cut to 1 UTF-16 unit → half a surrogate pair (TypeScript, QML)
```

The tests use accented letters, CJK and emoji, so a solution that counts bytes or UTF-16 units fails them.

## Graphemes are out of scope

Counting what a reader sees needs the Unicode segmentation rules. Only some languages have them built in: JavaScript has `Intl.Segmenter` (but QML doesn't), PHP has `grapheme_strlen` when the `intl` extension is loaded, and Python, Go, Rust and Lua need a third-party package. So the tests count code points, and one case in each exercise shows the difference: `"e\u0301"` looks like one letter but is two code points, and both count. A real UI that truncates names should cut by grapheme, so it never separates an accent from its letter.

Normalisation is a related trap: `é` can be one code point (U+00E9) or two (`e` + U+0301), and the two don't compare equal until both are normalised to the same form (NFC or NFD). The exercises don't normalise.

## Go deeper

- `learn compare text` puts every track's version side by side.
- [The Absolute Minimum Every Software Developer Must Know About Unicode](https://tonsky.me/blog/unicode/) covers code points, graphemes and normalisation in one read.
