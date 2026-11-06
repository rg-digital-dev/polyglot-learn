# Reading files in PHP

`file_get_contents($path)` reads a whole file into a string. It's one of PHP's older functions: when the file can't be read it doesn't throw, it emits a warning and returns `false`. So check before or after reading (`is_file()`, `is_readable()`, or `=== false`) and throw your own exception with the path in the message. `file($path, FILE_IGNORE_NEW_LINES)` returns the lines as an array, and `SplFileObject` reads line by line for large files.

```php
function firstLine(string $path): string
{
    if (!is_readable($path)) {
        throw new RuntimeException("can't read $path");
    }
    $text = file_get_contents($path);
    return trim(explode("\n", $text)[0]);
}

try {
    firstLine('does-not-exist.txt');
} catch (RuntimeException $e) {
    echo $e->getMessage(); // can't read does-not-exist.txt
}
```

<!-- from:typescript -->
## Coming from TypeScript
- No exception for a missing file: `false` plus a warning. Check and throw yourself.
- The `@` operator hides the warning, but then you still have to check for `false`; checking first is clearer.
- `explode("\n", $text)` is `text.split("\n")`, and `trim()` also removes `\r`.
<!-- /from -->

<!-- from:default -->
## Key points
- `file_get_contents()` returns `false` on failure; it doesn't throw.
- Throw an exception that names the file.
- `explode("\n", ...)` plus `trim()` handles `\r\n` endings.
<!-- /from -->

## Common mistake
Using the result without checking it:

```php
$lines = explode("\n", file_get_contents($path)); // with strict_types: a TypeError on false
```

## In this exercise
`readLines($path)` reads the file, throws an exception naming the path if it can't, and returns the trimmed lines that aren't blank or `#` comments. The tests pass the full path of files `learn` puts next to them.
