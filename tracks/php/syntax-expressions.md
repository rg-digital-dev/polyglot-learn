# Operators and Strings in PHP

Use `===` and `!==` to compare. The loose `==` converts types first, so `1 == "1"` and `null == false` are both true. Text is joined with `.`, not `+`. Double-quoted strings expand variables (`"Hi, {$name}!"`), and single-quoted strings do not. The logical operators are `&&`, `||` and `!`. `??` gives a fallback when a value is `null` or missing, `?:` when it is falsy, and `<=>` compares two values and returns `-1`, `0` or `1`. In PHP 8, `0 == "a"` is false, so a number no longer equals non-numeric text.

```php
<?php
declare(strict_types=1);

$name = "Ada";
echo "Hi, " . $name . "!\n";        // --> Hi, Ada!
echo "Hi, {$name}!\n";              // --> Hi, Ada!
echo 'Hi, $name!' . "\n";           // --> Hi, $name!
var_dump(1 === "1", 1 == "1");      // --> bool(false)  bool(true)
var_dump(null ?? "default");        // --> string(7) "default"
var_dump("" ?: "empty");            // --> string(5) "empty"
var_dump(5 <=> 3, 10 % 3, 2 ** 3);  // --> int(1)  int(1)  int(8)
echo trim("  x "), "\n";            // --> x
```

<!-- from:typescript -->
## Coming from TypeScript
- Join text with `.`, and append with `.=`. `+` is for numbers only.
- `===` works as you expect. Loose `==` is wilder than in JavaScript for strings: `"1" == "01"` is true.
- `??` and `?->` exist and work like `??` and `?.`. `**` is the power operator.
- There are no template literals. Use `"…{$var}…"` in double quotes.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- `&&`, `||` and `!` replace `and`, `or` and `not`. The words `and` and `or` exist, but bind looser than `=`.
- `+` does not join text, and `==` does not mean "same type and value". Use `.` and `===`.
- `trim($s)` is a function, not a method, and there is no `strip()`.
<!-- /from -->

<!-- from:go -->
## Coming from Go
- Mixing types is allowed: `"10" + 1` is `11`, and `1 == "1"` is true. Reach for `===` to get Go's strictness.
- Strings join with `.`, and double quotes can hold `{$var}`, so you rarely need `sprintf`.
<!-- /from -->

<!-- from:default -->
## Key points
- Prefer `===` and `!==`. Use `==` only when you want conversion.
- `.` joins text. `"Score: " + 5` throws a `TypeError`.
- `??` replaces a `null` or missing value, `?:` replaces any falsy one.
- `&&` and `||` short-circuit and return a real `bool`.
<!-- /from -->

## Common mistake
Using `and` where you meant `&&` in an assignment:

```php
$ok = true and false;    // $ok is true: parsed as ($ok = true) and false
$ok = true && false;     // $ok is false
```
