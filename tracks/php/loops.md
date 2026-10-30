# Loops in PHP

PHP has a C-style `for`, `while`, `do`/`while`, and `foreach`, which walks an
array either as values (`foreach ($xs as $x)`) or as key/value pairs
(`foreach ($xs as $k => $v)`). `continue` skips to the next iteration and
`break` leaves the loop; both accept a number to jump out of nested loops, as in
`break 2`.

```php
<?php
declare(strict_types=1);

/** @param list<int> $readings */
function firstSpike(array $readings, int $limit): ?int
{
    foreach ($readings as $index => $value) {
        if ($value <= $limit) {
            continue;
        }
        return $index;
    }
    return null;
}

$total = 0;
for ($i = 1; $i <= 3; $i++) {
    $total += $i;
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `foreach ($xs as $x)` is the counterpart of `for (const x of xs)`, and
  `foreach ($xs as $i => $x)` gives you the key as well.
- The loop variable is not block-scoped: `$x` still exists after the loop ends,
  holding the last value.
- `break 2` and `continue 2` replace labelled `break outer`.
- `count($xs)` is the length; there is no `.length` property.
<!-- /from -->

<!-- from:default -->
## Key points
- Use `foreach` to visit array elements and `for` for numeric ranges.
- `continue` and `break` take an optional level number for nested loops.
- Increment with `$i++` or `$sum += $x`.
<!-- /from -->

## Common mistake
Iterating by reference and then reusing the variable. After
`foreach ($xs as &$x)`, `$x` still points at the last element, so a second loop
over `$xs` with `$x` overwrites it. Call `unset($x)` right after a by-reference
loop, or avoid references altogether.

```php
foreach ($xs as &$x) { $x *= 2; }
unset($x);
```

## In this exercise
Implement `function sumOfMultiples(array $factors, int $limit): int` in
`sum-of-multiples/sum-of-multiples.php`. `$factors` is a `list<int>`.
