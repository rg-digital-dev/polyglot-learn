# Validation rules as attributes

Attributes (PHP 8.0) are structured metadata on classes, methods, properties and parameters, written `#[Name(arguments)]`. PHP does nothing with them by itself; code reads them through Reflection. Symfony's validator, Doctrine's mappings and Laravel's newer features all work this way, and so does this stage.

```php
<?php
declare(strict_types=1);

#[Attribute(Attribute::TARGET_PARAMETER | Attribute::TARGET_PROPERTY)]
final class Max
{
    public function __construct(public readonly int $value)
    {
    }
}

final class Order
{
    public function __construct(
        #[Max(10)] public readonly int $quantity,
        public readonly string $note = '',
    ) {
    }
}

$input = ['quantity' => '12'];
foreach ((new ReflectionClass(Order::class))->getConstructor()->getParameters() as $p) {
    foreach ($p->getAttributes(Max::class) as $attr) {
        $max = $attr->newInstance();                 // runs new Max(10)
        $value = (int) ($input[$p->getName()] ?? 0);
        if ($value > $max->value) {
            echo "{$p->getName()} must be at most {$max->value}\n";
        }
    }
    echo $p->getName(), ' defaults to ', var_export($p->isDefaultValueAvailable() ? $p->getDefaultValue() : null, true), "\n";
}
```

`#[Attribute(...)]` on a class makes it usable as an attribute and says where it may go. `getAttributes(Rule::class, ReflectionAttribute::IS_INSTANCEOF)` finds every attribute whose class implements `Rule`, so new rules work without touching the validator.

With promoted constructor parameters, an attribute on the parameter belongs to both the parameter and the property. The validator can read the rules from the constructor and then build the object with `new $class(...$args)`, passing the values by name.

<!-- from:typescript -->
## Coming from TypeScript
- Attributes look like TypeScript decorators (`@MaxLength(80)`), but they don't run any code by themselves. They're data until something reads them with Reflection.
- `$attr->newInstance()` is when the attribute's class is actually constructed, with the arguments written in the attribute.
- class-validator does the same job in TypeScript, using decorators and `reflect-metadata`.
<!-- /from -->

<!-- from:default -->
## Key points
- Declare attribute classes with `#[Attribute(targets)]`; read them with Reflection.
- Have rules implement one interface and find them with `IS_INSTANCEOF`.
- Report every error per field at once, so a user can fix the whole form in one go.
<!-- /from -->

## Common mistake
Running every rule on an empty optional field. `bio = ""` then fails `Length(min: 10)` even though the field wasn't required. Check whether the value is missing first: missing and optional means "use the default", not "check the length".

## In this stage
In `Validation.php`, make `Required`, `Length` and `Email` attributes, implement their `check` methods, and write `Validator::validate`.
