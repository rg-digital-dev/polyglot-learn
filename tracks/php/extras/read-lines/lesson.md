# Generators in PHP

A function that contains `yield` is a generator function. Calling it runs none of its body; it returns a `Generator` object. Each time a `foreach` (or `current()`/`next()`) asks for a value, the body runs until the next `yield`, hands that value out and pauses. `yield $key => $value` sets the key too; plain `yield $value` uses keys 0, 1, 2, and so on. `yield from $inner` passes on everything another generator (or array) yields, and evaluates to the inner generator's `return` value. A generator's own `return` value doesn't appear in the loop; read it with `getReturn()` once the generator has finished. Because nothing runs until asked, a generator can be infinite.

```php
<?php
declare(strict_types=1);

/** "a=1;b=2" yields 'a' => '1', 'b' => '2', and returns how many pairs. */
function pairs(string $spec): Generator
{
    $count = 0;
    foreach (explode(';', $spec) as $part) {
        [$key, $value] = explode('=', $part, 2);
        yield $key => $value;                  // a key and a value
        $count++;
    }
    return $count;                             // read with getReturn()
}

function settings(): Generator
{
    $n = yield from pairs('host=localhost;port=8080');  // delegate; gets the inner return
    $n += yield from pairs('debug=1');
    return $n;
}

function fibonacci(): Generator               // infinite, but lazy
{
    [$a, $b] = [0, 1];
    while (true) {
        yield $a;
        [$a, $b] = [$b, $a + $b];
    }
}

$g = settings();
foreach ($g as $key => $value) {
    echo "$key: $value\n";                     // host: localhost, port: 8080, debug: 1
}
echo $g->getReturn(), " settings\n";           // 3 settings

foreach (fibonacci() as $i => $f) {
    if ($i === 8) {
        break;                                 // stop asking; nothing more is computed
    }
    echo $f, ' ';                              // 0 1 1 2 3 5 8 13
}
echo "\n";
```

<!-- from:typescript -->
## Coming from TypeScript
- No `function*`: any `yield` in the body makes the function a generator. Declare the return type as `Generator`.
- `yield from` is `yield*`, and like `yield*` it evaluates to the inner generator's return value.
- A finished generator can't be restarted: a second `foreach` over the same object throws "Cannot traverse an already closed generator". Call the function again for a fresh one.
- `iterable` accepts both arrays and generators, like `Iterable<T>`.
<!-- /from -->

<!-- from:default -->
## Key points
- `yield` pauses the function and hands out a value; `yield $k => $v` also sets the key.
- `yield from` delegates to another generator and gives you its `return` value.
- `getReturn()` works only after the generator has finished.
- Stop asking (with `break` or `return`) and the generator does no more work.
<!-- /from -->

## Common mistake
Collecting with `iterator_to_array($gen)` when the keys repeat. It keeps the keys by default, so later values overwrite earlier ones: `iterator_to_array(concatLines(["a\nb", "c"]))` gives `[1 => 'c', 2 => 'b']`. Pass `false` as the second argument to get a plain list. And calling `getReturn()` too early throws `Exception: Cannot get return value of a generator that hasn't returned`.

## In this exercise
In `read-lines/read-lines.php`, write five generator functions: `readLines()` (numbered lines, returns the count), `nonBlank()` (filters any `iterable`), `concatLines()` (with `yield from`, returns the total), `naturals()` (infinite) and `take()`, which must stop right after the `$n`th item without pulling one more from its source.
