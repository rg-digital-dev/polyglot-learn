# Reading files in Python

`open(path, encoding="utf-8")` opens a text file, and `f.read()` reads all of it into a string; always pass the encoding, because the default depends on the system. Open files in a `with` block, which closes them when the block ends, also when something fails. When the file can't be opened, `open` raises an `OSError` subclass such as `FileNotFoundError`, and its message includes the path. `pathlib.Path(path).read_text(encoding="utf-8")` does the same in one call. `str.splitlines()` splits on `\n`, `\r\n` and a few more.

```python
def first_line(path: str) -> str:
    with open(path, encoding="utf-8") as f:
        return f.readline().strip()


try:
    first_line("does-not-exist.txt")
except OSError as e:
    print(e)   # [Errno 2] No such file or directory: 'does-not-exist.txt'
```

<!-- from:typescript -->
## Coming from TypeScript
- `open(...).read()` is `readFileSync(path, "utf8")`; both throw (raise) for a missing file, and both name the path.
- The `with` block closes the file for you; there's no file descriptor to manage.
- Pass `encoding="utf-8"`: unlike Node, Python's default depends on the operating system.
<!-- /from -->

<!-- from:default -->
## Key points
- `with open(path, encoding="utf-8") as f:` then `f.read()`.
- Errors are `OSError`s (`FileNotFoundError`, `PermissionError`) and name the file.
- Split on `"\n"` and `strip()` each line, or use `splitlines()`.
<!-- /from -->

## Common mistake
Leaving the file open:

```python
text = open(path).read()          # closed only when Python gets round to it
with open(path, encoding="utf-8") as f:
    text = f.read()               # fixed: closed at the end of the block
```

## In this exercise
`read_lines(path)` reads the file and returns the trimmed lines that aren't blank or `#` comments. Python's own error already names the file, so you can let it propagate. The tests pass the full path of files `learn` puts next to them.
