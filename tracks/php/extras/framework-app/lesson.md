# A container, the app, and a front controller

Classes ask for what they need in their constructors (`__construct(private Clock $clock)`), and something has to build them in the right order. A dependency injection container does it by reading the constructor's parameter types through Reflection. Symfony, Laravel and PHP-DI all autowire this way.

```php
<?php
declare(strict_types=1);

final class Clock
{
}

final class Report
{
    public function __construct(public readonly Clock $clock, public readonly string $title = 'Daily')
    {
    }
}

function build(string $class, array &$made = []): object
{
    if (isset($made[$class])) {
        return $made[$class];                   // each class built once
    }
    $args = [];
    foreach ((new ReflectionClass($class))->getConstructor()?->getParameters() ?? [] as $p) {
        $type = $p->getType();
        $args[] = $type instanceof ReflectionNamedType && !$type->isBuiltin()
            ? build($type->getName(), $made)     // a class: build that first
            : $p->getDefaultValue();             // anything else: its default
    }
    return $made[$class] = new $class(...$args);
}

$report = build(Report::class);
echo get_class($report->clock), ' ', $report->title, "\n";   // Clock Daily
```

A real container adds factories for things it can't work out (a database connection needs a DSN), errors that say what was missing, and cycle detection: A needs B, and B needs A, would otherwise recurse until PHP runs out of stack.

The front controller is the one PHP file the web server runs for every URL. It builds the app, turns PHP's request globals into a `Request`, and emits the `Response`. PHP's built-in server can run it:

```sh
php -S localhost:8000 public/index.php
```

<!-- from:typescript -->
## Coming from TypeScript
- This is what NestJS and InversifyJS do with `@Injectable()` and `reflect-metadata`; PHP's Reflection reads constructor types without any decorators.
- The front controller is the `server.listen` part of an Express app: everything else is plain objects you can test without a server.
- `header(...)` (a first-class callable, PHP 8.1) makes a Closure from a function, like `fn.bind(null)`.
<!-- /from -->

<!-- from:default -->
## Key points
- Ask for dependencies in the constructor; let the container build them.
- Build each service once; detect cycles and explain failures.
- Keep the front controller tiny: build the app, make a Request, emit the Response.
<!-- /from -->

## Common mistake
Calling `$container->get()` from inside your classes ("service locator"). Their real dependencies are then hidden, tests have to set up a whole container, and nothing tells you a class needs a mailer until it's sent its first email. Ask for dependencies in the constructor instead.

## In this stage
In `App.php`, implement `Container`, `App` and `emit`. `public/index.php` is ready: once the tests pass, run it with `php -S localhost:8000 public/index.php` and open `/hello/you`. The framework is done.
