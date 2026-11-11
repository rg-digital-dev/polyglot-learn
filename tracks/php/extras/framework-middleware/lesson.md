# Middleware: wrapping the handler

Middleware is code that runs around every request: logging, sessions, authentication, error pages. PSR-15 (the PHP standard behind Slim, Mezzio and Laminas) defines it as one method: take the request and the next handler, and return a response. Each middleware decides whether to call `$next` at all, and can change the request on the way in and the response on the way out.

```php
<?php
declare(strict_types=1);

interface Step
{
    public function run(string $input, Closure $next): string;
}

final class Shout implements Step
{
    public function run(string $input, Closure $next): string
    {
        return strtoupper($next($input));      // after: change the result
    }
}

final class Trim implements Step
{
    public function run(string $input, Closure $next): string
    {
        return $next(trim($input));            // before: change the input
    }
}

$core = fn(string $s) => "[$s]";
$steps = [new Shout(), new Trim()];            // the first is outermost

$next = $core;
foreach (array_reverse($steps) as $step) {
    $inner = $next;                            // capture this layer's next
    $next = fn(string $s) => $step->run($s, $inner);
}
echo $next('  hello  '), "\n";                 // [HELLO]
```

The chain is built from the inside out. Start with the core, then wrap it in the last middleware, then the one before, and so on. The first in the list ends up outermost: it sees the request first and the response last.

An interface is a promise of methods. `Shout` and `Trim` are interchangeable wherever a `Step` is expected, which is what lets the pipeline treat every middleware the same way.

<!-- from:typescript -->
## Coming from TypeScript
- Express middleware is `(req, res, next) => void` and changes `res` in place. PSR-15 middleware returns the response instead, so "after" code is just the line after `$next->handle()`.
- PHP arrow functions capture variables by value when they're created. That's why the loop copies `$next` into `$inner`: the closure keeps that copy, not the variable.
- `implements` works as in TypeScript, except that PHP checks it when the class is loaded, not just at compile time.
<!-- /from -->

<!-- from:default -->
## Key points
- A middleware gets the request and `$next`; whatever it returns is the response.
- Build the pipeline from the inside out; the first middleware is outermost.
- Catch exceptions in an outer middleware, so every error becomes a proper response.
<!-- /from -->

## Common mistake
Putting the error middleware last. It then wraps only the router, and an exception thrown by another middleware (a session store that's down, say) escapes as PHP's own fatal error page. Error handling goes first, so it wraps everything.

## In this stage
In `Middleware.php`, implement `CallableHandler`, `Pipeline` and the three middleware.
