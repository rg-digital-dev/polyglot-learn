# Reading files in Go

`os.ReadFile(path)` reads a whole file and returns `([]byte, error)`; convert with `string(data)`. Its errors are `*fs.PathError` values whose message includes the operation and the path (`open notes.txt: no such file or directory`), so you can usually return them as they are, or wrap them with `%w` for more context. `errors.Is(err, fs.ErrNotExist)` tells you the file wasn't there, whatever the operating system's wording. For large files, `bufio.NewScanner(file)` reads line by line.

```go
func firstLine(path string) (string, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return "", err // "open does-not-exist.txt: The system cannot find the file specified."
	}
	line, _, _ := strings.Cut(string(data), "\n")
	return strings.TrimSpace(line), nil
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Like `readFileSync`, but the error is a second return value instead of a throw.
- The result is bytes (`[]byte`); `string(data)` converts it.
- `errors.Is(err, fs.ErrNotExist)` replaces checking `e.code === "ENOENT"`.
<!-- /from -->

<!-- from:default -->
## Key points
- `data, err := os.ReadFile(path)`, then check `err` straight away.
- The error already names the file.
- `strings.Split(text, "\n")` plus `strings.TrimSpace` handles `\r\n` endings.
<!-- /from -->

## Common mistake
Ignoring the error and using the empty result:

```go
data, _ := os.ReadFile(path) // missing file: data is empty, and the caller never finds out
```

## In this exercise
`ReadLines(path)` reads the file and returns the trimmed lines that aren't blank or `#` comments. `go test` runs in the exercise's folder, so the tests pass just the file names of the files `learn` puts there.
