# Wrapping and inspecting errors

An error can carry another error inside it. `fmt.Errorf` with the `%w` verb builds a new message and **wraps** the original, so each layer of a program can add context ("reading app.conf: ...") without losing what went wrong underneath. Callers then look through the whole chain with two functions from the `errors` package. `errors.Is(err, target)` reports whether any error in the chain equals `target`; it is used with **sentinel errors**, package-level values such as `io.EOF` made with `errors.New`. `errors.As(err, &target)` finds the first error in the chain of a given type and stores it in `target`; it is used with **custom error types**, structs that carry details and have an `Error() string` method.

```go
package main

import (
	"errors"
	"fmt"
)

var ErrClosed = errors.New("account closed")

type LimitError struct {
	Limit, Asked int
}

func (e *LimitError) Error() string {
	return fmt.Sprintf("asked %d, limit is %d", e.Asked, e.Limit)
}

func withdraw(open bool, amount int) error {
	if !open {
		return fmt.Errorf("withdraw: %w", ErrClosed)
	}
	if amount > 500 {
		return fmt.Errorf("withdraw: %w", &LimitError{Limit: 500, Asked: amount})
	}
	return nil
}

func main() {
	err := fmt.Errorf("payday: %w", withdraw(true, 800))
	fmt.Println(err) // payday: withdraw: asked 800, limit is 500

	var le *LimitError
	if errors.As(err, &le) {
		fmt.Println("over by", le.Asked-le.Limit) // over by 300
	}
	fmt.Println(errors.Is(withdraw(false, 1), ErrClosed)) // true
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `errors.As` is the `instanceof` check, and it also looks inside wrapped errors, like following `error.cause` all the way down.
- `%w` is like `new Error(msg, { cause: err })`, except that the cause's message also ends up in the new message.
- Sentinels are compared by identity, not by message: two `errors.New("x")` values are different errors.
<!-- /from -->

<!-- from:default -->
## Key points
- Use `%w` to wrap, `%v` only when callers should not see the original error.
- Compare sentinels with `errors.Is`, not `==`, so wrapping doesn't break the check.
- Pass a pointer to `errors.As`: `var pe *ParseError` and then `errors.As(err, &pe)`.
- Return custom errors as pointers (`&ParseError{...}`) when the `Error` method has a pointer receiver.
<!-- /from -->

## Common mistake
Comparing a wrapped error with `==`:

```go
_, err := Lookup(lines, "port")    // returns fmt.Errorf("key %q: %w", key, ErrNotFound)
if err == ErrNotFound { ... }      // false: err is the wrapper, not the sentinel
if errors.Is(err, ErrNotFound) { ... } // fixed: checks every layer
```

`go vet` also warns if you use `%w` with something that isn't an error: `fmt.Errorf format %w has arg key of wrong type string`.

## In this exercise
Implement `ParseError.Error`, `Lookup`, `LookupIn` and `Describe` in `wrappederrors.go`. You'll need to import `fmt` and `strings`. The tests check the exact messages and that `errors.Is` and `errors.As` still find the original error through both layers of wrapping.
