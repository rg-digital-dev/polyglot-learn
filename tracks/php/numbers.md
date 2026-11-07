# Numbers in PHP

PHP has a 64-bit `int` and a `float`, and switches between them on its own. `/` returns an `int` when the division is exact and a `float` otherwise (`10 / 2` is `5`, `7 / 2` is `3.5`). `intdiv` truncates toward zero and `%` takes the dividend's sign; `fmod` does the same for floats. When an `int` result doesn't fit in 64 bits, it becomes a `float`. `round` rounds halves away from zero but always returns a `float`. The exercise files use `declare(strict_types=1)`, so returning a float from a function declared `: int` throws a `TypeError`.

```php
<?php
var_dump(10 / 2, 7 / 2);              // int(5) float(3.5)
var_dump(intdiv(-7, 2), -7 % 2);      // int(-3) int(-1)
var_dump(round(-2.5), (int) round(-2.5)); // float(-3) int(-3)
var_dump(PHP_INT_MAX + 1);            // float(9.223372036854776E+18)
var_dump(is_float(4294967296 * 4294967296)); // bool(true)
```

<!-- from:typescript -->
## Coming from TypeScript
- There are real integers, exact up to 2^63 − 1, not only doubles.
- `/` is JavaScript's `/` when it doesn't come out even; `intdiv` is `Math.trunc(a / b)`.
- `round(-2.5)` is `-3.0`, while `Math.round(-2.5)` is `-2`.
<!-- /from -->

<!-- from:default -->
## Key points
- `intdiv` and `%` truncate; `/` returns a float unless the result is a whole number.
- Integer overflow turns the result into a float instead of wrapping.
- `round` rounds halves away from zero and returns a float: cast it with `(int)`.
- PHP 8.4 adds a `RoundingMode` enum for other rules; the course supports PHP 8.1 and up, so the exercises don't need it.
<!-- /from -->

## Common mistake
Returning `round` or `/` directly from an `int` function:

```php
function half(int $n): int { return $n / 2; }
half(4);   // 2
half(5);   // TypeError: Return value must be of type int, float returned
```

## In these exercises
`wrapIndex` and `splitEvenly` need floor semantics, which `%` and `intdiv` don't give for negative numbers. `roundPercent` works with `intdiv` alone. `checkedMultiply` can rely on PHP turning an oversized product into a float.
