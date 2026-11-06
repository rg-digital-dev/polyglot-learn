# Error context in PHP

Every PHP exception can hold the exception that caused it: it's the third constructor argument, `$previous`, and `getPrevious()` returns it. So adding context means catching the low-level exception, then throwing a new one with a message that adds what you know and the old exception as `$previous`. Uncaught exceptions print the whole chain ("Next ..."), and code can walk it with `getPrevious()` in a loop. Catch the specific class you expect (`InvalidArgumentException`), not `\Throwable`, so programming errors aren't relabelled as bad input.

```php
function readRate(array $row, string $column): float
{
    if (!array_key_exists($column, $row)) {
        throw new InvalidArgumentException("$column: missing");
    }
    try {
        return parseRate($row[$column]);
    } catch (InvalidArgumentException $e) {
        throw new InvalidArgumentException("$column: " . $e->getMessage(), 0, $e);
    }
}

try {
    readRate(['vat' => 'lots'], 'vat');
} catch (InvalidArgumentException $e) {
    echo $e->getMessage();                // vat: not a rate: "lots"
    echo get_class($e->getPrevious());    // InvalidArgumentException
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `$previous` is `cause`: the third constructor argument (the second is an error code, usually `0`).
- `getPrevious()` returns it, and `null` at the end of the chain.
- `catch (InvalidArgumentException $e)` filters by class, which TypeScript can't do in the `catch` itself.
<!-- /from -->

<!-- from:default -->
## Key points
- `new SomeException("context: " . $e->getMessage(), 0, $e)` adds context and keeps the cause.
- Catch the specific exception class you're wrapping.
- `getPrevious()` walks back to the original.
<!-- /from -->

## Common mistake
Forgetting `$previous`, or catching too much:

```php
} catch (\Throwable $e) {                       // also catches TypeErrors from bugs
    throw new InvalidArgumentException('bad');   // and drops both the reason and the cause
}
```

## In this exercise
`Numbers\parseNumber` throws `InvalidArgumentException`s that say why. `loadSetting` finds the key, catches that exception, and throws a new one with the key in front and the original as `$previous`.
