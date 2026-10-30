# Iterators in PHP

Most PHP data processing works on whole arrays with the `array_*` functions: `array_map`, `array_filter`, `array_slice`, `array_keys` and friends take an array and return a new one. The sort functions (`usort`, `uasort`, `uksort`, `arsort` and so on) sort in place, and differ in whether they keep the keys. For lazy sequences, a function that contains `yield` is a generator: calling it returns a `Generator`, which `foreach` consumes one value at a time.

```php
<?php
declare(strict_types=1);

/** @return Generator<int, string> */
function nonEmptyLines(string $text): Generator
{
    foreach (explode("\n", $text) as $i => $line) {
        if (trim($line) !== '') {
            yield $i + 1 => trim($line);
        }
    }
}

$rows = [];
foreach (nonEmptyLines("b 3\n\na 7\nc 7") as $lineNo => $line) {
    [$name, $score] = explode(' ', $line);
    $rows[] = ['name' => $name, 'score' => (int) $score, 'line' => $lineNo];
}

usort($rows, fn ($x, $y) => ($y['score'] <=> $x['score']) ?: strcmp($x['name'], $y['name']));
echo implode(', ', array_map(fn ($r) => "{$r['name']}@{$r['line']}", $rows)), "\n";
// a@3, c@4, b@1
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no method chaining; calls nest inside out, as in `array_slice(array_keys($m), 0, 3)`. Mind the argument order: `array_map($fn, $array)` but `array_filter($array, $fn)`.
- `<=>` returns -1, 0 or 1, and `?:` falls through to a second key.
- `usort` throws the keys away; `uasort` keeps them but its comparator sees only values, and `uksort` sees only keys.
- `yield` alone makes a function a generator; there is no `function*`. `iterator_to_array` collects one into an array.
<!-- /from -->

<!-- from:default -->
## Key points
- Count, sort with a two-key comparator, `array_slice`, keep the words.
- `usort` is stable since PHP 8.0.
- Generators produce values lazily, which helps with large or streamed input.
<!-- /from -->

## Common mistake
Forgetting that numeric-string keys become integers. After counting, the word `"3"` is the key `3`, so `array_keys` returns `int` values and a strict comparison with `"3"` fails. Convert back with `array_map('strval', $words)`.

## In this exercise
Implement `function topWords(string $text, int $n): array` in `top-words/top-words.php`, returning a list of strings.
