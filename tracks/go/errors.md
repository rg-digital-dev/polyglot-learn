# Errors in Go

In Go, errors are ordinary values of the built-in `error` interface. A function that can fail returns its result plus an `error` as the last value, and returns `nil` for the error on success. The caller checks `if err != nil` right away and either handles it or returns it upward, often wrapped with context using `fmt.Errorf("...: %w", err)`. `errors.New` makes a simple error. `panic` exists, but it is meant for bugs, not for bad input.

```go
func ParsePercent(input string) (int, error) {
	s := strings.TrimSuffix(strings.TrimSpace(input), "%")
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("not a number: %q", s)
	}
	if n < 0 || n > 100 {
		return 0, errors.New("percent must be between 0 and 100")
	}
	return n, nil
}
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `throw`/`try`/`catch` for expected failures; the error is a second return value.
- You must check `err` yourself after every call; nothing unwinds the stack for you.
- `strconv.Atoi` is stricter than `parseInt`: `"80a"` is an error. It still accepts `+80` and `-1`.
- Out-of-range numbers return an error rather than losing precision.
<!-- /from -->

<!-- from:default -->
## Key points
- Return `(value, nil)` on success and `(zero, err)` on failure.
- Check `if err != nil` immediately after each call that can fail.
- Use `fmt.Errorf` with `%w` to add context while keeping the original error.
<!-- /from -->

## Common mistake
Discarding the error with `_`:

```go
n, _ := strconv.Atoi(s)    // "abc" silently becomes 0
n, err := strconv.Atoi(s)  // fixed: check it
if err != nil {
	return 0, err
}
```

## In this exercise
Implement `func ParsePort(input string) (int, error)`. Invalid input must return a non-nil error; the tests only check that it is an error, and a panic fails them.
