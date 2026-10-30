# Errors in PHP

Modern PHP reports failures with exceptions. You `throw` an object that
implements `Throwable`, and handle it with `try`/`catch`/`finally`. Your own code
usually throws `Exception` subclasses from the standard library, such as
`InvalidArgumentException` or `RangeException`, while the engine throws `Error`
subclasses like `TypeError`. One `catch` can list several types with `|`.

```php
<?php
declare(strict_types=1);

function parseRatio(string $text): float
{
    $parts = explode('/', $text);
    if (count($parts) !== 2 || !is_numeric($parts[0]) || !is_numeric($parts[1])) {
        throw new InvalidArgumentException("not a ratio: '$text'");
    }
    if ((float) $parts[1] === 0.0) {
        throw new DomainException("denominator is zero in '$text'");
    }
    return (float) $parts[0] / (float) $parts[1];
}

try {
    parseRatio('3/0');
} catch (InvalidArgumentException | DomainException $e) {
    echo $e->getMessage(), "\n";
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `throw`, `try`, `catch` and `finally` work the same way, but `catch` must
  name a type: `catch (Exception $e)`, or `catch (Throwable $e)` for everything.
- Only objects implementing `Throwable` can be thrown; there is no
  `throw "oops"`.
- `Exception` and `Error` are separate branches, so `catch (Exception $e)` does
  not catch a `TypeError`.
- Like TypeScript, PHP has no checked exceptions; document them with `@throws`.
<!-- /from -->

<!-- from:default -->
## Key points
- Throw the most specific standard exception that fits, with a clear message.
- Catch only where you can handle the failure; let the rest propagate.
- `Throwable` is the common interface of `Exception` and `Error`.
<!-- /from -->

## Common mistake
Using a cast to validate. `(int) '80a'` is `80`, and `(int)` on a digit string
too large for `int` silently returns `PHP_INT_MAX`. Check the characters and
the length first:

```php
if (!ctype_digit($s)) {
    throw new InvalidArgumentException("not a number: '$s'");
}
```

## In this exercise
Implement `function parsePort(string $input): int` in `parse-port/parse-port.php`.
Throw an exception, such as `InvalidArgumentException`, for bad input.
