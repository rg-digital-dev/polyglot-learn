# Error context in Go

Go builds error context into the standard library: `fmt.Errorf` with the `%w` verb makes a new error whose message is your text plus the old message, and which still *contains* the old error. `errors.Is(err, target)` and `errors.As(err, &target)` look through every layer of wrapping, so a caller can check for a specific cause (a sentinel like `ErrNotANumber`, or an error type) no matter how much context was added on top. Use `%v` instead of `%w` when you want the text but deliberately don't want callers to depend on the cause.

```go
var ErrNoSuchUser = errors.New("no such user")

func findUser(id string) error {
	return fmt.Errorf("%w: %q", ErrNoSuchUser, id)
}

func loadProfile(id string) error {
	if err := findUser(id); err != nil {
		return fmt.Errorf("loading profile: %w", err)
	}
	return nil
}

func main() {
	err := loadProfile("u7")
	fmt.Println(err)                           // loading profile: no such user: "u7"
	fmt.Println(errors.Is(err, ErrNoSuchUser)) // true, through both layers
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `%w` is `{ cause: err }` and the message prefix in one: the wrapped error keeps the cause, and its `Error()` text includes it.
- `errors.Is` walks the chain of causes for you, like checking `e.cause` repeatedly.
- There's no `try`/`catch`: you wrap at the `if err != nil` right after the call.
<!-- /from -->

<!-- from:default -->
## Key points
- `fmt.Errorf("context: %w", err)` adds context and keeps the original.
- `errors.Is` and `errors.As` see through any number of wraps.
- Exported sentinel errors (`var ErrX = errors.New(...)`) let callers check the cause.
<!-- /from -->

## Common mistake
Using `%v` (or `err.Error()`) where you meant `%w`:

```go
return fmt.Errorf("port: %v", err) // the text is the same, but errors.Is can't see the cause
return fmt.Errorf("port: %w", err) // the cause stays reachable
```

## In this exercise
`numbers.ParseNumber` returns errors that say why. `LoadSetting` finds the key and wraps any error with `fmt.Errorf("%s: %w", key, err)`, so the message mentions both the key and the reason.
