# Traits in PHP

A class can extend only one parent. A `trait` is how PHP shares code
sideways: it holds properties and methods, and `use TraitName;` inside a class
copies them into that class as if you had typed them there. `$this` in a trait
method is the object of the using class, and every object gets its own copy of
the trait's properties. A trait can declare an `abstract` method, which the
using class must implement; that is how the trait asks the class for
something. When two traits used by one class define the same method, PHP
refuses to load the class until you choose one with `insteadof`, and you can
keep the other under a new name with `as`.

```php
<?php
declare(strict_types=1);

trait Counts
{
    private int $count = 0;

    public function bump(): void
    {
        $this->count++;
    }

    public function report(): string
    {
        return "{$this->count} calls";
    }
}

trait Logs
{
    /** @var list<string> */
    private array $lines = [];

    abstract protected function prefix(): string;   // the using class must supply it

    public function log(string $msg): void
    {
        $this->lines[] = $this->prefix() . $msg;
    }

    public function report(): string
    {
        return implode("\n", $this->lines);
    }
}

final class Mailer
{
    use Counts, Logs {
        Logs::report insteadof Counts;   // Mailer::report() is the Logs one
        Counts::report as callCount;     // the Counts one under another name
    }

    protected function prefix(): string
    {
        return '[mail] ';
    }

    public function send(string $to): void
    {
        $this->bump();
        $this->log("sent to $to");
    }
}

$m = new Mailer();
$m->send('ana@example.com');
$m->send('bo@example.com');
echo $m->report(), "\n";      // two lines: "[mail] sent to ..."
echo $m->callCount(), "\n";   // 2 calls
print_r(class_uses($m));      // Array ( [Counts] => Counts [Logs] => Logs )
```

<!-- from:typescript -->
## Coming from TypeScript
- A trait is like a mixin, but it is resolved when the class is loaded, not
  at runtime, and it is not a type: you can't write `function f(HasTags $x)`.
  Pair a trait with an interface when callers need a type.
- Name clashes are an error, not "last one wins". You resolve them
  explicitly in the `use` block.
- `as` can also change visibility: `Counts::bump as protected;`.
<!-- /from -->

<!-- from:default -->
## Key points
- `use A, B;` copies both traits' properties and methods into the class.
- An `abstract` method in a trait is a requirement on the using class.
- Resolve clashes with `A::m insteadof B;` and keep the other with
  `B::m as otherName;`.
- `class_uses($obj)` lists the traits a class uses directly.
<!-- /from -->

## Common mistake
Using two traits that both define a method, without a `use` block:

```php
final class Mailer
{
    use Counts, Logs;
    // Fatal error: Trait method Logs::report has not been applied as
    // Mailer::report, because of collision with Counts::report
}
```

The same goes for a missing abstract method: "Class Mailer contains 1
abstract method and must therefore be declared abstract or implement the
remaining method (Mailer::prefix)". Both are fatal errors when the file
loads, so no test runs at all until you fix them.

## In this exercise
In `timestamps-trait/timestamps-trait.php`, fill in the methods of the traits
`HasTimestamps` and `HasTags`. `Bookmark` already uses `HasTags`. Then make
`Note` use both traits, keep `describe()` from `HasTimestamps`, expose the
`HasTags` one as `describeTags()`, implement the abstract `now()`, and write
`summary()`.
