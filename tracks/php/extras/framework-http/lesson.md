# Requests and responses as readonly values

This project builds a tiny web framework: the path one HTTP request takes from arriving to being answered. It's one folder that grows over eight stages, one file per stage, with no Composer and no packages. Tests build requests in code and call your framework directly, so no web server is needed until the last stage.

Everything starts with two values. Modern PHP frameworks (PSR-7, Symfony's HttpFoundation in spirit) treat the request and the response as values that don't change. Two PHP 8.1 features make that easy: `readonly` properties and backed enums.

```php
<?php
declare(strict_types=1);

enum Size: string
{
    case Small = 'S';
    case Large = 'L';
}

final class Shirt
{
    public function __construct(
        public readonly Size $size,
        public readonly string $colour = 'black',
    ) {
    }

    /** A copy with one thing changed: the original stays as it was. */
    public function withColour(string $colour): self
    {
        return new self($this->size, $colour);
    }
}

$a = new Shirt(Size::tryFrom('L') ?? throw new InvalidArgumentException('no such size'));
$b = $a->withColour('red');
echo $a->colour, ' ', $b->colour, ' ', $b->size->value, "\n";   // black red L

try {
    $a->colour = 'blue';
} catch (Error $e) {
    echo $e->getMessage(), "\n";   // Cannot modify readonly property Shirt::$colour
}
```

Constructor promotion (`public readonly Size $size` in the constructor's parameters) declares and assigns the property in one go. A backed enum has a fixed set of cases, each with a string or int value. `Size::from()` throws for an unknown value; `tryFrom()` returns null.

Why immutable? A middleware that adds a header gets a new response back, so it can't change one that other code still holds. Every change is visible as an assignment.

<!-- from:typescript -->
## Coming from TypeScript
- `public readonly` is TypeScript's `readonly`, but enforced at run time: assigning throws an `Error`.
- Constructor promotion is TypeScript's parameter properties (`constructor(public readonly size: Size)`).
- A backed enum is a string enum, but with methods of its own and `match` checking: `Method::Get->isSafe()`.
<!-- /from -->

<!-- from:default -->
## Key points
- `public readonly` promoted properties: set once in the constructor, never again.
- "With" methods return a new object with one thing changed.
- Backed enums for fixed sets of values; `tryFrom` to parse safely.
<!-- /from -->

## Common mistake
Storing headers exactly as given and looking them up with `$headers['Content-Type']`. HTTP header names aren't case-sensitive, and clients send `content-type`, `Content-Type` and `CONTENT-TYPE`. Normalise on the way in, or compare with `strcasecmp`.

## In this stage
In `Http.php`, make `Request` and `Response` readonly and implement every method marked TODO. Each later stage adds a file to this folder and builds on these two classes.
