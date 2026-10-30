# Modules in PHP

PHP files are not modules on their own; two separate features share the job.
`require_once` loads another file and runs it, at most once however many files
ask for it. A `namespace` declaration at the top of a file puts its functions,
classes and constants under a prefix such as `Text\`. To call a namespaced
function by its short name, import it with `use function`; classes use plain
`use`. PHP's autoloading only covers classes, so files of functions are always
loaded with `require_once` (Composer's `files` setting does it for you).

```php
<?php
// text/format.php
declare(strict_types=1);

namespace Text;

function shout(string $s): string
{
    return strtoupper($s) . '!';
}
```

```php
<?php
// text/main.php
declare(strict_types=1);

require_once __DIR__ . '/format.php';

use function Text\shout;

echo shout('deploy done'), "\n"; // DEPLOY DONE!
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `export`: everything in a namespace is public.
- `use` only creates a short name; it never loads a file. Loading is the job
  of `require_once`.
- Build paths from `__DIR__`, the current file's folder. A bare relative path
  is resolved against the include path and the working directory instead.
- Inside a namespace, an unknown function such as `strlen()` falls back to the
  global one; class names do not (`new \DateTime`).
<!-- /from -->

<!-- from:default -->
## Key points
- Helper file: `namespace Words;` right after `declare`.
- Using file: `require_once __DIR__ . '/words.php';`, then
  `use function Words\split;`.
- Prefer `require_once` to `require`, so loading a file twice does not
  redeclare its functions.
<!-- /from -->

## Common mistake
Calling the function without importing it. In `main.php` above, `shout('x')`
without the `use function` line fails with "Call to undefined function
shout()", because the function's full name is `Text\shout`.

## In this exercise
First implement `function split(string $text): array` in `slugify/words.php`
(namespace `Words`); its tests run first. Then implement
`function slugify(string $title): string` in `slugify/slugify.php`, which loads
the helper with `require_once` and imports it with `use function Words\split;`.
