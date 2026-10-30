# Collections and sorting in PHP

PHP's sort functions work on the array you pass by reference and return `true`, not a new array. `sort`/`rsort` order values and reindex the keys, `asort`/`arsort` keep each key with its value, `ksort`/`krsort` order by key, and `usort` takes a comparison callback. The spaceship operator `<=>` returns -1, 0 or 1, which is exactly what a comparator needs. `array_slice` copies part of an array.

```php
<?php
declare(strict_types=1);

/** @param list<array{name: string, wins: int}> $players */
function ranking(array $players): array
{
    usort($players, fn ($a, $b) => [$b['wins'], $a['name']] <=> [$a['wins'], $b['name']]);
    return array_column($players, 'name');
}

print_r(ranking([
    ['name' => 'Bo', 'wins' => 2],
    ['name' => 'Al', 'wins' => 5],
])); // [0] => Al, [1] => Bo
```

<!-- from:typescript -->
## Coming from TypeScript
- `sort($xs)` returns `true`; the sorted result is `$xs` itself afterwards.
- Because arrays are passed by value, sorting a parameter inside a function does not touch the caller's array. No defensive copy is needed.
- Default sorting compares numbers as numbers, so `[10, 9, 1]` sorts to `[1, 9, 10]`, unlike JavaScript's string-based default.
- `<=>` on arrays compares element by element, handy for tie-breakers.
<!-- /from -->

<!-- from:default -->
## Key points
- Sort functions modify their argument and return `bool`.
- `rsort` sorts descending; `usort` with `fn ($a, $b) => $b <=> $a` does too.
- `array_slice($xs, 0, $n)` takes at most `$n` items.
- Inside a function, a parameter array is a copy of the caller's array.
<!-- /from -->

## Common mistake
Assigning the return value of a sort function:

```php
$sorted = rsort($scores);  // $sorted is true
rsort($scores);            // now $scores is sorted
```

## In this exercise
Implement `function topScores(array $scores, int $n): array` in `top-scores/top-scores.php`, taking and returning a `list<int>`.
