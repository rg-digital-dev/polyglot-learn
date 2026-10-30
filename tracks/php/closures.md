# Closures in PHP

PHP has two kinds of anonymous function, and both produce `Closure` objects. A `function () use ($x) { ... }` closure sees only the outer variables listed in `use`, copied when the closure is created. `use (&$x)` captures the variable by reference instead, so the closure can change it and the change survives between calls. An arrow function, `fn ($y) => $x + $y`, captures the outer variables it uses automatically, always by value, and its body is a single expression.

```php
<?php
declare(strict_types=1);

function makeLogger(string $prefix): Closure
{
    $lines = [];
    return function (string $msg) use ($prefix, &$lines): int {
        $lines[] = "$prefix $msg";
        return count($lines);
    };
}

$log = makeLogger('[app]');
$log('started');
echo $log('ready'), "\n"; // 2

$rate = 21;
$withVat = fn (int $cents): int => intdiv($cents * (100 + $rate), 100);
echo $withVat(1000), "\n"; // 1210
```

<!-- from:typescript -->
## Coming from TypeScript
- Capture is explicit. A JavaScript closure sees the live outer variable; a PHP `function` closure sees only what `use` lists.
- `use ($x)` is a snapshot. `use (&$x)` is the one that behaves like JavaScript capture.
- `fn` looks like `=>` but captures by value, so a `fn` cannot update outer state.
- There is no function-type syntax like `() => number`. The return type is `Closure`; describe the rest in PHPDoc: `@return Closure(): int`.
<!-- /from -->

<!-- from:default -->
## Key points
- State that a closure changes needs `function () use (&$var)`.
- Use `fn` for short read-only callbacks, such as `array_map(fn ($x) => $x * 2, $xs)`.
- Each call of the factory creates a fresh local variable, so every closure gets its own state.
<!-- /from -->

## Common mistake
Capturing a counter by value. The code runs, but each call starts again from the captured copy:

```php
$n = 0;
$next = function () use ($n) { return ++$n; };
echo $next(), ' ', $next(); // 1 1, not 1 2
```

## In this exercise
Implement `function makeCounter(int $start, int $step): Closure` in `make-counter/make-counter.php`. The tests create one or two counters and call each of them several times.
