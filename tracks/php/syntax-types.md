# Types and Annotations in PHP

PHP checks types at each function call, not on variables. You write a type on a parameter, a return value or a property (`function f(int $n): int`), and PHP enforces it when the function runs. You cannot write `int $x = 1;` on a local variable: that is a parse error. The basic types are `int`, `float`, `string`, `bool`, `array` and `null`. `?string` means "string or `null`", and `int|string` means either. The course's stubs start with `declare(strict_types=1);`. Under it, passing `"3"` to an `int` parameter throws a `TypeError`. Without it, PHP converts `"3"` to `3` and carries on. One conversion survives strict mode: an `int` passed to a `float` parameter becomes a `float`.

```php
<?php
declare(strict_types=1);

function add(int $a, int $b): int { return $a + $b; }
function half(float $x): float { return $x / 2; }

try {
    echo add("3", 4);
} catch (TypeError $e) {
    echo "TypeError\n";           // --> TypeError
}
var_dump(half(3));                // --> float(1.5)
var_dump(7 / 2, intdiv(7, 2));    // --> float(3.5)  int(3)
var_dump(gettype(null));          // --> string(4) "NULL"
```

<!-- from:typescript -->
## Coming from TypeScript
- Types are checked while the program runs, by PHP itself. There is no separate checker and no build step.
- That check covers only parameters, return values and properties. `$x = "a"` after `$x = 1` is fine.
- There is no `any`; a parameter with no type accepts anything. `mixed` says so on purpose.
- `?string` replaces `string | null`, and `int|string` is the union form.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Unlike Python annotations, these are enforced: a wrong type throws a `TypeError` at the call.
- Without `strict_types=1`, PHP converts scalars (`"3"` becomes `3`), which is why the stubs turn it on.
- `None` is `null`, and `?string` is the short form of `string|null`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Lua values carry their own type but nothing checks the arguments. In PHP you can ask for a type, and a mismatch throws.
- `"10" + 1` is `11` in both languages, but `"Score: " + 5` throws a `TypeError` in PHP.
<!-- /from -->

<!-- from:default -->
## Key points
- Types go on parameters, returns and properties, never on local variables.
- `strict_types=1` stops string-to-int conversion in calls. It applies to calls made from the file that declares it.
- `/` gives a `float` unless both sides divide evenly (`6 / 3` is `int(2)`). Use `intdiv` for integer division.
- `void` means a function returns nothing, and `?int` allows `null`.
<!-- /from -->

## Common mistake
Forgetting that a return type is checked too. `function r(): int { return "5"; }` throws `TypeError` under `strict_types`, and so does a function with a return type that never returns anything.
