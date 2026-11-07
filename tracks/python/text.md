# Text in Python

A Python `str` is a sequence of code points. `len(s)` counts them, `s[i]` is one code point (as a string of length 1), slices cut between code points, and `for ch in s` visits each one. So Python can't split `é` or an emoji in half. Bytes are a separate type: `s.encode()` gives the UTF-8 `bytes`, and `b.decode()` turns them back.

```python
s = "Zoë 😀"
print(len(s))               # 5 code points
print(len(s.encode()))      # 9 bytes in UTF-8
print(s[-1], hex(ord(s[-1])))   # 😀 0x1f600
print([ch for ch in s][:3]) # ['Z', 'o', 'ë']
```

<!-- from:typescript -->
## Coming from TypeScript
- `len(s)` is `[...s].length`, not `s.length`: an emoji counts once.
- Indexing and slicing work by code point, so `s[:n]` never leaves half a surrogate pair.
- There is no `Intl.Segmenter`; counting graphemes needs a package such as `grapheme` or `regex` (`\X`).
<!-- /from -->

<!-- from:default -->
## Key points
- `len`, indexing, slicing and `for` all work in code points.
- `s.encode()` gives UTF-8 bytes; use it when you need the byte length.
- `unicodedata.normalize("NFC", s)` merges `e` + combining accent into `é`.
<!-- /from -->

## Common mistake
Code points aren't what a reader sees:

```python
s = "e\u0301"               # shows as é
print(len(s))               # 2
import unicodedata
print(len(unicodedata.normalize("NFC", s)))   # 1
```

The tests expect the 2 here: count code points, don't normalise.

## In these exercises
`char_count(text)` returns a dict from each code point to how often it occurs. `truncate(text, limit)` keeps text that fits and otherwise cuts to `limit - 1` code points plus `"…"`. Plain `len` and slices are already right; watch the `limit` of 0.
