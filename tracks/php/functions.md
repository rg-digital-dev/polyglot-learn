# Functions and strings in PHP

A PHP function is declared with `function`, can live at the top level of a file,
and is callable from anywhere once that file is loaded. Parameters and return
values take type declarations such as `string` or `int`. Strings are byte
sequences: double-quoted strings expand variables (`"Hi $name"` or
`"Hi {$user->name}"`), single-quoted strings do not, and `.` joins two strings.

```php
<?php
declare(strict_types=1);

function badge(string $user, int $points = 0): string
{
    $label = strtoupper($user);
    return "{$label} ({$points} pts)";
}

echo badge('ada'), "\n";         // ADA (0 pts)
echo badge(points: 3, user: 'bo'); // BO (3 pts)
```

<!-- from:typescript -->
## Coming from TypeScript
- Type declarations are checked at runtime, not by a compiler: a wrong type
  throws a `TypeError` when the call happens.
- Without `declare(strict_types=1);` at the top of the calling file, PHP
  quietly converts scalars, so `badge('x', '5')` would pass `5` as an int.
- Variables start with `$`, and `+` is only for numbers. Use `.` or
  interpolation to build strings; `'a' + 'b'` is a `TypeError`.
- Named arguments (`badge(points: 3, user: 'bo')`) exist since PHP 8.0.
- `strlen` counts bytes, not characters: `strlen('é')` is 2.
<!-- /from -->

<!-- from:default -->
## Key points
- Declare parameter and return types; with `strict_types=1` they are enforced
  without silent conversion.
- Use double quotes and `{$var}` to interpolate; single quotes are literal.
- `trim`, `strtoupper` and friends return new strings and never modify their
  argument.
<!-- /from -->

## Common mistake
Expecting variables to expand inside single quotes:

```php
echo 'Hello, $name!';  // prints: Hello, $name!
echo "Hello, $name!";  // prints: Hello, Ada!
```

## In this exercise
Implement `function greet(string $name): string` in `hello/hello.php`. The test
file uses `strict_types=1`, so return a real `string`.
