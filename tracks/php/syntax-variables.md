# Variables and Constants in PHP

Every variable starts with `$` and is created by assigning to it. There is no declaration keyword and no type on the variable, so the same name can hold an `int` and then a string. A constant is made with `const NAME = value;` at the top level of a file (or inside a class), or with `define('NAME', value)`. A constant has no `$`, and it cannot be changed. `const` inside a function is a parse error. Only functions create a new scope: a variable made inside an `if` or a loop is still there after it.

```php
<?php
declare(strict_types=1);

$count = 1;
$count = "two";            // allowed: the name has no fixed type
var_dump($count);          // --> string(3) "two"

const MAX = 3;
define('LIMIT', 5);
echo MAX + LIMIT, "\n";    // --> 8

function f() { $x = 1; }
f();
var_dump(isset($x));       // --> bool(false): $x lived inside f

if (true) { $inner = 5; }
echo $inner, "\n";         // --> 5: an if block is not a scope
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `let` or `const` before a variable. Assigning with `$x = 1;` creates it.
- A function cannot see outer variables. Inside `function f() { … }`, `$count` from the file is a new, empty name. Pass it as a parameter, or use `use ($count)` on a closure.
- Constants are for fixed values only (`const MAX = 3;`). `const` is not the way to make a normal variable.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Like Python, there is no block scope, and functions do not see the file's variables. Unlike Python, you need `$` every time.
- `const` is real in PHP: reassigning `MAX` is an error, not a convention.
- A constant has no `$`: `MAX`, not `$MAX`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- `$x = 1` is always local to the current function. Nothing like Lua's accidental global exists.
- Functions do not see the file's variables, which is the opposite of how Lua upvalues work.
<!-- /from -->

<!-- from:default -->
## Key points
- Variables start with `$`; constants do not.
- A variable can change type. Only function parameters, return values and properties carry types.
- Arrays are copied on assignment: after `$ys = $xs;` changing `$ys` leaves `$xs` alone.
- `const` works at the top level or in a class, not inside a function.
<!-- /from -->

## Common mistake
Expecting a function to read a variable from the file around it:

```php
$rate = 2;
function double(int $n): int { return $n * $rate; }  // $rate is undefined here
```

Pass `$rate` as a parameter instead.
