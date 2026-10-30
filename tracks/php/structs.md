# Structs in PHP

PHP has no struct type; a record is a small class whose properties hold the
data. Constructor property promotion (PHP 8.0) declares and assigns a property
directly in the constructor's parameter list, and `readonly` (8.1) lets a
property be set once and never again. A `final` class with promoted readonly
properties is the modern value-object idiom. Named arguments, such as
`new Money(cents: 5, currency: 'EUR')`, keep calls with several fields readable.

```php
<?php
declare(strict_types=1);

final class Money
{
    public function __construct(
        public readonly int $cents,
        public readonly string $currency,
    ) {}

    public function plus(Money $other): Money
    {
        return new Money(cents: $this->cents + $other->cents, currency: $this->currency);
    }
}

$a = new Money(cents: 1250, currency: 'EUR');
$b = $a->plus(new Money(cents: 99, currency: 'EUR'));
echo $b->cents, ' ', $b->currency, "\n"; // 1349 EUR
```

<!-- from:typescript -->
## Coming from TypeScript
- Promotion looks like TypeScript's parameter properties
  (`constructor(public readonly cents: number)`), and in PHP it is the
  everyday style.
- Classes are nominal: only an object made with `new Money` passes a `Money`
  type hint, not an array or another class with the same properties.
- `readonly` is enforced at runtime; a second assignment throws an `Error`.
- Objects are passed by handle, like JavaScript objects, while arrays are
  copied. `==` compares two objects of the same class property by property;
  `===` asks whether they are the same instance.
<!-- /from -->

<!-- from:default -->
## Key points
- `final class` with promoted `public readonly` properties is an immutable
  record.
- Named arguments can come in any order and must match the parameter names.
- To "change" a readonly object, build a new one.
<!-- /from -->

## Common mistake
Dividing with `/`. `999 * 10 / 100` is the float `99.9`, and under
`strict_types` passing it to an `int` parameter throws a `TypeError`. Use
`intdiv(999 * 10, 100)`, which gives `99`.

## In this exercise
Implement `function orderTotal(array $items, int $discountPercent): OrderSummary`
in `order-total/order-total.php`. `$items` is a list of `LineItem` objects; both
classes are already in the stub. The tests compare the result's class and
property values.
