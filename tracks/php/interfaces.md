# Interfaces in PHP

An `interface` lists public methods without bodies. A class that declares
`implements` must define every one of them with a compatible signature, or PHP
refuses to load the class. Parameters and return types can then name the
interface, and any object of an implementing class is accepted. A class can
implement several interfaces but extend only one parent class.

```php
<?php
declare(strict_types=1);

interface Notifier
{
    public function send(string $message): string;
}

final class EmailNotifier implements Notifier
{
    public function __construct(private readonly string $address) {}

    public function send(string $message): string
    {
        return "mail to {$this->address}: $message";
    }
}

final class LogNotifier implements Notifier
{
    public function send(string $message): string
    {
        return "log: $message";
    }
}

foreach ([new EmailNotifier('ops@example.com'), new LogNotifier()] as $n) {
    echo $n->send('disk full'), "\n";
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Typing is nominal: a class satisfies an interface only if it says
  `implements`. Having the right methods is not enough.
- Interfaces exist at runtime, so `$x instanceof Notifier` works and type
  hints are checked on every call.
- `array` cannot say "array of Notifier"; write `@param list<Notifier>` in
  PHPDoc so your editor knows.
<!-- /from -->

<!-- from:default -->
## Key points
- Declare the contract with `interface` and fulfil it with `implements`.
- Type hint the interface, not the concrete classes.
- `usort` is stable since PHP 8.0, so equal elements keep their order.
<!-- /from -->

## Common mistake
Leaving out a return type the interface declares. If the interface has
`send(string $message): string` and the class writes
`send(string $message)`, PHP stops with "Declaration of ... must be compatible
with ...". Copy the signature exactly.

## In this exercise
In `shapes/shapes.php`, fill in `area()` and `label()` on `Rect` and `Square`,
then implement `function describeAll(array $shapes): array`, which returns a
list of strings. Call only the `Shape` methods; don't check the class.
