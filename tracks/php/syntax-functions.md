# Functions in PHP

A function is declared with `function name(type $param): returnType { … }`. A top-level function can be called from anywhere once its file is loaded. Parameters can have defaults (`int $b = 10`), and since PHP 8.0 you can pass arguments by name (`add(b: 5, a: 1)`). A function returns one value, so to return two you return an array and unpack it with `[$q, $r] = …`. Arrow functions (`fn($x) => $x * 2`) capture the outer scope on their own, but a full `function () use ($base) { … }` closure must list what it uses.

```php
<?php
declare(strict_types=1);

function add(int $a, int $b = 10): int { return $a + $b; }
echo add(1), " ", add(1, 2), " ", add(b: 5, a: 1), "\n";   // --> 11 3 6

function divmod(int $a, int $b): array { return [intdiv($a, $b), $a % $b]; }
[$q, $r] = divmod(7, 2);
echo "$q $r\n";                                            // --> 3 1

$base = 10;
$addBase = function (int $x) use ($base): int { return $x + $base; };
$arrow = fn(int $x): int => $x + $base;
echo $addBase(1), " ", $arrow(2), "\n";                    // --> 11 12
```

<!-- from:python -->
## Coming from Python
- A function must `return` its result. The last expression is not returned for you.
- Types are enforced: `add("1", 2)` throws a `TypeError` under `strict_types`.
- Functions cannot see the file's variables. Pass them in, or use `use ($var)` or an arrow function.
- `fn` takes a single expression, like `lambda`.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Every parameter starts with `$`, and the type comes before the name: `int $a`.
- Arrow functions capture outer variables by value, automatically. Full `function` closures need `use`.
- `void` return means no value, and `?int` or `int|null` marks an optional result.
<!-- /from -->

<!-- from:go -->
## Coming from Go
- Only one value comes back. Return an array and unpack it with `[$a, $b] = f();`.
- Errors are exceptions you `throw` and `catch`, not extra return values.
<!-- /from -->

<!-- from:default -->
## Key points
- Always `return` the value. Printing it does not make a function return it.
- Under `strict_types=1`, a wrong argument or return type throws `TypeError`.
- Scalar arguments are copied: changing `$n` inside a function does not change the caller's variable.
- Calling a function with too few arguments throws `ArgumentCountError`.
<!-- /from -->

## Common mistake
Echoing the result instead of returning it. With a return type, a function that falls off the end throws a `TypeError` such as `typed(): Return value must be of type int, none returned`.
