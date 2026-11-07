# Text in PHP

PHP strings are plain bytes with no encoding attached. `strlen`, `$s[$i]`, `substr` and `str_split` all work in bytes, so they split multi-byte UTF-8 characters. For code points there are three families: the PCRE functions with the `u` flag (always available), `iconv_strlen`/`iconv_substr` (built in on most installs), and the `mb_*` functions from the `mbstring` extension, which is common but not always enabled (on Windows it needs `extension=mbstring` in `php.ini`). `php -m` lists what's loaded.

```php
<?php
$s = "Zoë 😀";
echo strlen($s), "\n";                                  // 9 bytes
$chars = preg_split('//u', $s, -1, PREG_SPLIT_NO_EMPTY);
echo count($chars), "\n";                               // 5 code points
echo $chars[4], "\n";                                   // 😀
echo iconv_substr($s, 0, 3, 'UTF-8'), "\n";             // Zoë
```

<!-- from:typescript -->
## Coming from TypeScript
- `strlen` counts bytes, not UTF-16 units: `strlen("é")` is 2.
- `preg_split('//u', ...)` is `[...s]`: one string per code point.
- Grapheme functions (`grapheme_strlen`) need the `intl` extension; there's no built-in like `Intl.Segmenter`.
<!-- /from -->

<!-- from:default -->
## Key points
- `strlen`, `substr` and `$s[$i]` count bytes.
- `preg_split('//u', $s, -1, PREG_SPLIT_NO_EMPTY)` splits into code points anywhere.
- `mb_strlen`/`mb_substr` and `iconv_strlen`/`iconv_substr` count code points when their extension is loaded.
<!-- /from -->

## Common mistake
Cutting with `substr`:

```php
substr("résumé", 0, 2);              // "r\xC3": half of é, shows as �
iconv_substr("résumé", 0, 2, 'UTF-8'); // "ré"
```

## In these exercises
`charCount($text)` returns an array from each code point to its count. `truncate($text, $limit)` counts and cuts in code points: split with `preg_split('//u', ...)`, then `count`, `array_slice` and `implode`. The tests don't need `mbstring`.
