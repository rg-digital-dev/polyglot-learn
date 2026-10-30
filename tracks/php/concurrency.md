# Concurrency in PHP

A PHP script runs on one thread, and plain PHP has no way to start more (the `parallel` extension needs a thread-safe build and is rarely installed). PHP 8.1 added Fibers: a `Fiber` wraps a function that can pause itself with `Fiber::suspend()` and be continued later with `$fiber->resume()`. Only one fiber runs at a time, and each one decides when to give way: cooperative multitasking. Libraries such as AMPHP build non-blocking I/O on top of fibers. On their own, fibers only let several tasks take turns.

```php
<?php
declare(strict_types=1);

$fiber = new Fiber(function (string $name): string {
    $reply = Fiber::suspend("hello $name"); // pause and hand a value out
    return "got $reply";
});

echo $fiber->start('ada'), "\n";    // hello ada
$fiber->resume('thanks');           // continues after the suspend
var_dump($fiber->isTerminated());   // bool(true)
echo $fiber->getReturn(), "\n";     // got thanks
```

Real waiting with `sleep()` blocks the whole process, fibers included, so fibers cannot make sleeps overlap. That is why this exercise uses a virtual clock: time is a counter that your scheduler advances.

<!-- from:typescript -->
## Coming from TypeScript
- There is no event loop behind your code. Nothing resumes a fiber unless you call `resume()`; you write the scheduler.
- `Fiber::suspend()` is like `await`, but it works from any depth of ordinary function calls; no caller needs to be marked `async`.
- `sleep()` and blocking I/O stop every fiber, unlike `setTimeout`.
<!-- /from -->

<!-- from:default -->
## Key points
- `start()` once, then `resume()` until `isTerminated()`; read the result with `getReturn()`.
- Start every fiber before resuming any of them.
- Keep fibers in an array keyed by input index, so results stay in order.
<!-- /from -->

## Common mistake
Resuming a fiber that has already finished. It throws `FiberError` ("Cannot resume a fiber that is not suspended"). Check `isTerminated()` first.

## In this exercise
`run-all/run-all.php` provides `Clock` and `function task(int $ms): int`; leave them unchanged. Implement `function runAll(array $delays): array` so the tasks take turns on the virtual clock instead of running one after another. The tests check the results and that `Clock::$now` ends at about the largest delay, not the sum.
