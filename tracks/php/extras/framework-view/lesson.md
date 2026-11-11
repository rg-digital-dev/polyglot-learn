# Templates with output buffering and escaping

PHP started as a template language, and a plain `.php` file is still a fine template: HTML with `<?= … ?>` for values and `<?php foreach … ?>` for loops. Two things turn it into a template engine: capturing the output into a string, and escaping every value that came from a user.

```php
<?php
declare(strict_types=1);

function e(mixed $value): string
{
    return htmlspecialchars((string) $value, ENT_QUOTES | ENT_SUBSTITUTE, 'UTF-8');
}

$template = sys_get_temp_dir() . '/greeting.php';
file_put_contents($template, '<p>Hi, <?= e($name) ?>!</p>');

function render(string $file, array $vars): string
{
    ob_start();                      // from here on, output is collected...
    try {
        (static function () use ($file, $vars): void {
            extract($vars);          // $vars['name'] becomes $name
            require $file;
        })();
        return (string) ob_get_clean();   // ...and returned, not printed
    } catch (Throwable $e) {
        ob_end_clean();              // don't leave the buffer open
        throw $e;
    }
}

echo render($template, ['name' => '<script>steal()</script>']), "\n";
// <p>Hi, &lt;script&gt;steal()&lt;/script&gt;!</p>
```

`ob_start()` starts collecting everything that would be printed; `ob_get_clean()` stops and returns it. Buffers nest. A template that throws halfway leaves its buffer open unless something closes it, and the half-page then shows up in front of the next response. Note the level before and clean up in `finally`.

Escaping is what stops cross-site scripting (XSS). A comment containing `<script>` that's printed as it is runs in every visitor's browser. `htmlspecialchars` turns `<`, `>`, `&` and quotes into entities. `ENT_QUOTES` covers single quotes too, which matters inside attributes. Escape on output, every time, and mark the rare raw output (`$content` in a layout) clearly.

<!-- from:typescript -->
## Coming from TypeScript
- In JSX, `{name}` is escaped automatically and `dangerouslySetInnerHTML` is the exception. A PHP template is the other way round, so `e()` has to be a habit.
- Output buffering is like capturing `process.stdout.write` into a string.
- `extract($vars)` makes array keys into local variables, a bit like destructuring every prop at once.
<!-- /from -->

<!-- from:default -->
## Key points
- `ob_start()` and `ob_get_clean()` turn printed output into a returned string.
- Escape every value with `htmlspecialchars(…, ENT_QUOTES | ENT_SUBSTITUTE, 'UTF-8')`.
- Close any buffer you opened, even when the template throws.
<!-- /from -->

## Common mistake
Escaping when saving instead of when printing. The database then holds `&amp;lt;`, values get escaped twice, and output that isn't HTML (a JSON API, an email subject) shows entities. Store the raw text and escape for the place it's going.

## In this stage
In `View.php`, implement `e`, `View::render` and `View::layout`.
