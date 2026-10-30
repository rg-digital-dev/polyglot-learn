# Readonly value objects in PHP

A value object is defined by its data, not by its identity: two `Money` objects for 5 euros are the same amount. The usual way to build one is to make it immutable. A `readonly` property (PHP 8.1) can be written once, from inside the class, and never again; any later write throws `Error`. Combined with constructor property promotion (`public readonly int $cents` in the constructor's parameter list), the class needs no separate property list or assignments. Validate in the constructor, so an invalid object can't exist, and write "withers" that return a modified copy instead of setters.

```php
<?php
declare(strict_types=1);

final class Duration
{
    public function __construct(
        public readonly int $seconds,          // promoted and readonly
    ) {
        if ($seconds < 0) {
            throw new InvalidArgumentException("negative duration: $seconds");
        }
    }

    public function plus(Duration $other): Duration
    {
        return new Duration($this->seconds + $other->seconds);  // a new object
    }

    public function withSeconds(int $seconds): Duration       // a "wither"
    {
        return new Duration($seconds);
    }

    public function equals(Duration $other): bool
    {
        return $this->seconds === $other->seconds;
    }
}

$a = new Duration(90);
$b = $a->plus(new Duration(30));
echo $a->seconds, ' ', $b->seconds, "\n";              // 90 120
var_dump($b->equals(new Duration(120)));               // bool(true)
var_dump($b === new Duration(120));                    // bool(false): different objects

try {
    $a->seconds = 5;
} catch (Error $e) {
    echo $e->getMessage(), "\n";   // Cannot modify readonly property Duration::$seconds
}
```

Newer versions add shortcuts. PHP 8.2 allows `final readonly class Duration`, which makes every property readonly. PHP 8.5 adds `clone($this, ['seconds' => $s])`, which copies an object with some readonly properties changed. This track only requires 8.1, so the exercise works with neither.

<!-- from:typescript -->
## Coming from TypeScript
- `readonly` is enforced at runtime, not only by the type checker: the write throws `Error: Cannot modify readonly property ...`.
- A readonly property must have a type, and has no default value; it is set in the constructor.
- `==` compares two objects property by property, and `===` checks that they are the same object. An explicit `equals()` method says what you mean.
- PHP has no `bigint` and no decimal type in the core. Keep money as integer cents and use `intdiv()` and `%`, never floats.
<!-- /from -->

<!-- from:default -->
## Key points
- Promote constructor parameters with `public readonly` and validate them in the constructor body.
- To "change" a value, return `new self(...)` with the new data.
- Integer cents: split `abs($c)` with `intdiv()` and `%`, pad with `sprintf('%02d', ...)`, and add the sign yourself. Splitting a negative amount directly gives `0.-5` for -5 cents.
<!-- /from -->

## Common mistake
Writing a wither as clone-then-assign:

```php
$copy = clone $this;
$copy->cents = $cents;   // Error: Cannot modify readonly property Money::$cents
return $copy;
```

The clone's properties are already initialised, so they are locked too. Call the constructor instead (`return new Money($cents, $this->currency);`), which also runs the validation again.

## In this exercise
In `money-value/money-value.php`, finish the class `Money` with readonly `int $cents` and `string $currency`: validate the currency in the constructor (three uppercase letters, else `InvalidArgumentException`), then implement `withCents()`, `add()`, `equals()`, `format()` and `allocate()`. Every method returns a new `Money` and leaves the original alone.
