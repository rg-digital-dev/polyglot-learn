# Maps in PHP

PHP has one collection type for both lists and maps: the `array`, an ordered
hash map whose keys are `int` or `string`. `$m['key'] = 1` adds or replaces an
entry, `isset($m['key'])` or `array_key_exists('key', $m)` tests for one, and
`foreach ($m as $key => $value)` visits entries in insertion order. Arrays are
values: assigning or passing one gives the receiver its own copy.

```php
<?php
declare(strict_types=1);

/** @return array<string, int> */
function tally(array $colors): array
{
    $seen = [];
    foreach ($colors as $c) {
        $seen[$c] = ($seen[$c] ?? 0) + 1;
    }
    return $seen;
}

print_r(tally(['red', 'blue', 'red'])); // [red] => 2, [blue] => 1
```

<!-- from:typescript -->
## Coming from TypeScript
- An associative `array` plays the role of both `Record<string, number>` and
  `Map`; there is no separate map type in the core language.
- Keys that look like decimal integers are converted to `int`: `$m['7'] = 1`
  stores key `7`, so `array_keys` returns `[7]`, not `['7']`.
- Reading a missing key returns `null` and emits a warning. Use `?? 0` for a
  default.
- Arrays are copied on assignment (lazily, copy-on-write), unlike JavaScript
  objects, which are shared by reference.
<!-- /from -->

<!-- from:default -->
## Key points
- One `array` type serves as list and map; document which one with PHPDoc
  (`list<int>`, `array<string, int>`).
- `$m[$k] = ($m[$k] ?? 0) + 1;` is the counting idiom.
- Decimal-integer string keys such as `'42'` become `int` keys.
<!-- /from -->

## Common mistake
Incrementing a key that does not exist yet. `$counts[$w]++` on a missing key
works but emits `Warning: Undefined array key`. Supply the default explicitly:

```php
$counts[$w] = ($counts[$w] ?? 0) + 1;
```

## In this exercise
Implement `function wordCount(string $text): array` in `word-count/word-count.php`,
returning `array<string, int>`. A word like `"1"` will become key `1`; that is
expected, and the tests compare maps after sorting their keys.
