# Cleanup in PHP

`try { ... } finally { ... }` is PHP's cleanup tool: the `finally` block runs when the `try` block is left, whether by `return`, by falling off the end, or by an exception passing through (which then continues upward). Keep the resources in an array as you open them, and let the `finally` close whatever is in it, so a failure halfway closes exactly the ones that were opened. Objects with a `__destruct` method are cleaned up when the last reference goes away, but relying on that for files or connections makes the timing hard to see; an explicit `finally` is clearer.

```php
function sendAll(array $hosts, string $message): void
{
    $connections = [];
    try {
        foreach ($hosts as $host) {
            $connections[] = connect($host); // may throw
        }
        foreach ($connections as $c) {
            $c->send($message); // may throw
        }
    } finally {
        foreach (array_reverse($connections) as $c) {
            $c->close();
        }
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- The same `try`/`finally` as in TypeScript; there's no `using`.
- A `return` inside `try` still runs `finally` before the function returns.
- Don't also close inside the `try`: the `finally` would close them a second time.
<!-- /from -->

<!-- from:default -->
## Key points
- Open into an array inside `try`, close the array in `finally`.
- `finally` runs on return, on normal completion and on exceptions.
- Close in one place only.
<!-- /from -->

## Common mistake
Closing at the end of the `try` instead of in `finally`:

```php
try {
    $opened[] = open($name);
    $data = $opened[0]->read(); // throws: the close below never runs
    $opened[0]->close();
} catch (RuntimeException $e) { /* ... */ }
```

## In this exercise
`Resource\open()` (from `resource.php`) returns a resource with `read()` and `close()`. `readAll` opens them all inside a `try`, reads each, and closes everything in the `finally`.
