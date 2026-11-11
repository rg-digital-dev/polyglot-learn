# A router with regular expressions and closures

A router maps a method and a path to the code that answers it. The usual trick, used by FastRoute and most frameworks, is to compile each route pattern into a regular expression with named groups, then try them in order.

```php
<?php
declare(strict_types=1);

function compile(string $pattern): string
{
    // "{name}" becomes a named group for one segment; everything else is literal.
    $regex = preg_replace_callback(
        '/\{(\w+)\}|[^{]+/',
        fn(array $m) => isset($m[1]) ? "(?P<{$m[1]}>[^/]+)" : preg_quote($m[0], '#'),
        $pattern,
    );
    return '#^' . $regex . '$#';
}

$routes = [
    [compile('/users/{name}'), fn(array $p) => "profile of {$p['name']}"],
    [compile('/about'), fn() => 'about us'],
];

foreach (['/users/ada', '/about', '/users/ada/edit'] as $path) {
    foreach ($routes as [$regex, $handler]) {
        if (preg_match($regex, $path, $m)) {
            $params = array_filter($m, 'is_string', ARRAY_FILTER_USE_KEY);   // only the named groups
            echo $path, ' -> ', $handler($params), "\n";
            continue 2;
        }
    }
    echo $path, " -> 404\n";
}
```

`(?P<name>…)` names a group, so `preg_match` puts the captured text in `$m['name']` as well as under its number. `preg_quote` escapes characters like `.` and `+` that would otherwise mean something in a regex. `^…$` makes the pattern match the whole path, not just part of it.

PHP is happy to call a closure with more arguments than it declares, so a handler can take `(Request $r, array $params)`, or only what it needs. `Closure::fromCallable()` turns any callable (a function name, `[$object, 'method']`, a closure) into a `Closure`.

A path that exists but not for this method should get **405 Method Not Allowed** with an `Allow` header, not 404. It tells the client the URL is right and the method is wrong.

<!-- from:typescript -->
## Coming from TypeScript
- This is what Express's `app.get('/users/:name', handler)` does inside; path-to-regexp compiles patterns the same way.
- `fn(array $p) => …` is an arrow function, and like JavaScript's it captures outer variables automatically, by value.
- A handler returning a string, an array or a Response is a union type, `Response|string|array`, checked when the function returns.
<!-- /from -->

<!-- from:default -->
## Key points
- Compile patterns to anchored regexes with named groups once, when routes are added.
- Collect the methods of routes whose path matched, for the 405's `Allow` header.
- Accept several return types from handlers and convert them in one place.
<!-- /from -->

## Common mistake
Leaving out the anchors, so `#/posts/(?P<id>\d+)#` also matches `/posts/42/edit` and `/old/posts/1`. Without `^` and `$`, a regex matches anywhere in the string.

## In this stage
In `Router.php`, implement `get`, `post`, `add` and `handle`. `Handler` is the interface every request handler will implement from now on.
