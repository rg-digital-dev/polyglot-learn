# Blocks and Control Flow in PHP

PHP marks blocks with braces. `if` needs parentheses around the condition, and `else if` can be written `elseif`. For lists use `foreach ($xs as $x)`, and for counting use a C-style `for ($i = 0; $i < 10; $i++)`. There are also `while` and `do … while`. `break` and `continue` work as usual. To pick between values, PHP has `match` (PHP 8.0+), which compares strictly, returns a value, never falls through and throws `UnhandledMatchError` when nothing fits. The older `switch` compares loosely and does fall through.

```php
<?php
declare(strict_types=1);

foreach (["a" => 1, "b" => 2] as $key => $value) {
    echo "$key=$value ";               // --> a=1 b=2
}
echo "\n";

for ($i = 0; $i < 3; $i++) {
    if ($i === 1) { continue; }
    echo $i;                           // --> 02
}
echo "\n";

$day = 3;
echo match ($day) {
    1, 7          => "weekend",
    2, 3, 4, 5, 6 => "weekday",
    default       => "?",
}, "\n";                               // --> weekday

switch ("5") { case 5: echo "loose match\n"; break; }            // --> loose match
switch (1) { case 1: echo "one "; case 2: echo "two "; break; }  // --> one two
echo "\n";
```

<!-- from:python -->
## Coming from Python
- Blocks need braces and every condition needs parentheses. Indentation means nothing.
- `foreach ($xs as $i => $x)` gives key and value in one go, so there is no `enumerate`. Over a list, the key is the position.
- `match` looks like Python 3.10's `match`, but each arm is an expression and there is no pattern destructuring.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- `foreach ($xs as $x)` replaces `for…of`. Over a map, `$k => $v` gives key and value.
- `switch` falls through and compares loosely (`"5"` matches `case 5`). Reach for `match` instead.
- `$i` from a `for` loop is still there after it: there is no block scope.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- `continue` and `switch` both exist in PHP.
- A loop variable stays alive after the loop.
- Braces replace `then … end` and `do … end`.
<!-- /from -->

<!-- from:default -->
## Key points
- Braces mark blocks, `;` ends statements, and conditions sit in parentheses.
- `foreach` is the loop for arrays; `for` is for counting.
- `match` is strict and returns a value, so prefer it to `switch`.
- A missing `break` in a `switch` runs the next case as well.
<!-- /from -->

## Common mistake
Leaving out `default` in a `match`. A value with no arm throws `UnhandledMatchError: Unhandled match case 9` instead of passing quietly.
