# A File in PHP

A PHP file starts with `<?php`. Everything after it is code, and each statement ends with `;`. The course's stubs put `declare(strict_types=1);` on the next line. It switches on strict typing for that file: a function with an `int` parameter then rejects the string `"3"` with a `TypeError` instead of quietly converting it. It must be the first statement in the file. Comments are `// …`, `# …` and `/* … */`, and a `/** … */` block above a function describes it (the stubs use `@param` and `@return`). A top-level `function` is visible to the tests with no extra marker. Print with `echo`, or `var_dump` to see a value's type.

```php
<?php
declare(strict_types=1);

// A comment. So is # this, and /* this */.
function greet(string $name): string
{
    return "Hello, " . trim($name) . "!";
}

echo greet("  Ada "), "\n";   // --> Hello, Ada!
$total = 1 + 2;
var_dump($total);             // --> int(3)
```

<!-- from:python -->
## Coming from Python
- Blocks use braces, not indentation, and every statement needs a `;`. A missing `;` is a parse error.
- Variables start with `$`. Function names do not.
- `echo` is a keyword, not a function: `echo $x;` needs no parentheses.
<!-- /from -->

<!-- from:javascript -->
## Coming from JavaScript
- `declare(strict_types=1);` is not like `"use strict"`. It only controls whether scalar arguments are converted to the declared type.
- Names are looked up by `$`: `$name` is the variable, `name` is a constant or function.
<!-- /from -->

<!-- from:default -->
## Key points
- Start with `<?php`, then `declare(strict_types=1);`.
- End each statement with `;`.
- Edit the body of the stub's function and leave its first line alone.
- With `strict_types=1`, passing a string where `int` is declared throws a `TypeError`. The one exception: an `int` is still accepted where `float` is declared.
<!-- /from -->

## Common mistake
Putting any statement, such as an `echo`, before `declare(strict_types=1);`. PHP stops with a fatal error: it must be the first statement after `<?php`.
