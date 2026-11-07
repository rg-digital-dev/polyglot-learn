# Reading files

Reading a whole text file is a one-liner in every language here. What differs is how each one tells you the file couldn't be read, and whether the message says *which* file:

| Track | Read a whole file | When it can't be read | Names the file? |
|---|---|---|---|
| Rust | `std::fs::read_to_string(path)` | `Err(io::Error)` | No: add the path yourself |
| Go | `os.ReadFile(path)` | `err` (a `*fs.PathError`) | Yes |
| TypeScript | `readFileSync(path, "utf8")` from `node:fs` | throws (`ENOENT`, ...) | Yes |
| Python | `open(path, encoding="utf-8").read()` | raises an `OSError` | Yes |
| PHP | `file_get_contents($path)` | returns `false` *and* emits a warning | Only in the warning: check and throw yourself |
| Lua | `io.open(path)` then `file:read("a")` | `nil, message` | Yes |

QML has no file access of its own (that's done in C++), so this exercise isn't on the QML track.

## Lines

Files written on Windows end lines with `\r\n`, elsewhere with `\n`, and the last line may or may not end with one. Splitting on `\n` and trimming each line handles all of it: the trim removes a trailing `\r` along with spaces. Splitting on `\r\n` alone breaks on Unix files, and splitting on both separately invents empty lines.

## Common mistake

Assuming the read worked:

```text
text = read(path)        // PHP: false; Lua: nil
for line in split(text)  // fails here, far from the cause, with a confusing message
```

Check right after the read, and fail with a message that names the file.

## Go deeper

- `learn compare files` puts every track's version side by side.
- Reading everything at once is fine for small files. For large ones, every language can also read line by line (`BufReader::lines`, `bufio.Scanner`, `readline`, `fgets`, `file:lines()`), which keeps memory use flat.
