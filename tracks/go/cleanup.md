# Cleanup in Go

`defer f()` schedules a call for when the surrounding *function* returns, whichever `return` that is (and also when it panics). Deferred calls run last in, first out. The pattern is to defer the close right after a successful open, before anything else can fail: `f, err := os.Open(name); if err != nil { return err }; defer f.Close()`. A `defer` inside a loop is fine for a handful of resources that should all stay open until the function returns; for thousands, move the loop body into its own function so each one closes per iteration.

```go
func copyFirstLine(from, to string) error {
	src, err := os.Open(from)
	if err != nil {
		return err
	}
	defer src.Close()

	dst, err := os.Create(to)
	if err != nil {
		return err // src is closed by its defer
	}
	defer dst.Close()

	line, err := bufio.NewReader(src).ReadString('\n')
	if err != nil && err != io.EOF {
		return err // both are closed, dst first
	}
	_, err = dst.WriteString(line)
	return err
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `defer` is a `finally` you write next to the open instead of at the end, and it's per function, not per block.
- Several defers run in reverse order, like several `using` declarations.
- Defer *after* the error check: deferring `Close` on a nil value would panic.
<!-- /from -->

<!-- from:default -->
## Key points
- `defer r.Close()` right after the successful open.
- Deferred calls run when the function returns, last one first.
- In a loop, every iteration's defer waits for the end of the function.
<!-- /from -->

## Common mistake
Closing at the end instead of deferring:

```go
r, err := Open(name)
if err != nil {
	return nil, err
}
data, err := r.Read()
if err != nil {
	return nil, err // r stays open
}
r.Close()
```

## In this exercise
`Open` (in `resource.go`, same package) returns a `*Resource` with `Read` and `Close`. `ReadAll` opens them all, deferring each close, then reads; every return then closes what was opened.
