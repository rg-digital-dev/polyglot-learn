# Conditionals in PHP

PHP branches with `if`/`elseif`/`else`, the ternary `?:`, `switch`, and the
`match` expression (PHP 8.0+), which compares strictly and returns a value. The
boolean operators are `&&`, `||` and `!`, and `%` gives the integer remainder.
Use `===` and `!==` for comparisons; they check both type and value.

```php
<?php
declare(strict_types=1);

function shippingCost(int $grams, bool $express): int
{
    if ($grams <= 0) {
        return 0;
    }
    $base = match (true) {
        $grams < 500  => 4,
        $grams < 2000 => 7,
        default       => 12,
    };
    return $express ? $base * 2 : $base;
}

var_dump(shippingCost(1200, false)); // int(7)
```

<!-- from:typescript -->
## Coming from TypeScript
- `if`, `else`, `? :`, `&&`, `||` and `===` read the same. Note `elseif` is
  usually written as one word.
- `match` is like a `switch` that returns a value, uses `===`, has no
  fall-through, and throws `UnhandledMatchError` when nothing matches.
- PHP also has `and` / `or` keywords, but they bind looser than `=`. Stick with
  `&&` and `||`.
- `%` works on integers (operands are converted to `int`); use `fmod` for floats.
<!-- /from -->

<!-- from:default -->
## Key points
- `$x % $n === 0` tests whether `$x` is divisible by `$n`.
- `&&` and `||` short-circuit and always produce a `bool`.
- `switch` compares loosely (`==`); `match` doesn't.
- A boolean expression can be returned as is.
<!-- /from -->

## Common mistake
Using `and` in an assignment. It binds looser than `=`, so the second operand
is evaluated and then thrown away:

```php
$ok = true and false;   // $ok is true: parsed as ($ok = true) and false
$ok = true && false;    // $ok is false
```

## In this exercise
Implement `function leapYear(int $year): bool` in `leap-year/leap-year.php`.
Return a real `bool`; the tests compare with `===`.
