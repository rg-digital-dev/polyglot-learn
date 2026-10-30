# Enums in PHP

An `enum` (PHP 8.1) is a type with a fixed set of values called cases. A
**pure** enum has cases only. A **backed** enum, `enum Level: int` or
`enum Suit: string`, gives every case a scalar value, and gets two static
methods for free: `from($value)` returns the case or throws `ValueError`, and
`tryFrom($value)` returns the case or `null`. Every enum also has `cases()`,
which lists the cases in declaration order. Cases are singleton objects, so
compare them with `===`. Enums can have methods, static methods, constants and
interfaces, but no properties and no `new`. A `match` over `$this` that lists
every case needs no `default` arm.

```php
<?php
declare(strict_types=1);

enum Level: int
{
    case Debug = 10;
    case Info = 20;
    case Error = 40;

    public const DEFAULT = self::Info;

    public function tag(): string
    {
        return match ($this) {          // every case listed: no default needed
            self::Debug => 'DBG',
            self::Info => 'INF',
            self::Error => 'ERR',
        };
    }

    public function atLeast(Level $min): bool
    {
        return $this->value >= $min->value;
    }
}

foreach (Level::cases() as $level) {
    echo $level->name, ' = ', $level->value, ' ', $level->tag(), "\n";
}

$fromConfig = Level::tryFrom(30) ?? Level::DEFAULT;   // 30 is not a case
echo $fromConfig->name, "\n";                          // Info
var_dump(Level::from(40) === Level::Error);            // bool(true)
var_dump(Level::Debug->atLeast(Level::DEFAULT));       // bool(false)
```

<!-- from:typescript -->
## Coming from TypeScript
- A PHP enum is closer to a union of string literals with methods attached
  than to a TS `enum`: there is no reverse mapping, and a case is not its
  value. `Level::Info === 20` is `false`; use `Level::Info->value`.
- `$case->name` is the case name as a string, `$case->value` the backing
  value (backed enums only).
- Enum types are checked at runtime, so a `function f(Suit $s)` can never
  receive an arbitrary string.
<!-- /from -->

<!-- from:default -->
## Key points
- Pure enum for labels, backed enum (`: int` or `: string`) when the value is
  stored or parsed.
- `tryFrom()` for input that may be wrong, `from()` when a bad value is a bug.
- Put behaviour on the enum as methods; `match ($this)` keeps every case in
  one place.
- Enum constants can point at a case: `const DEFAULT = self::Info;`.
<!-- /from -->

## Common mistake
Leaving a case out of a `match` with no `default` arm. PHP doesn't check this
when it loads the file; it fails only when the missing case shows up:
`UnhandledMatchError: Unhandled match case Suit::Clubs`. Likewise,
`Suit::from('X')` throws `ValueError: "X" is not a valid backing value for
enum Suit`. For user input, use `tryFrom()` and handle `null`.

## In this exercise
In `card-suits/card-suits.php` the cases are already declared. Fill in the
methods: `Suit::color()`, `Suit::symbol()` and `Suit::fromLetter()`,
`Rank::label()`, then `Card::label()`, `Card::parse()` (throw
`InvalidArgumentException` for bad input), `Card::beats()` using
`Suit::TRUMP`, and `function fullDeck(): array`.
