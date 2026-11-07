# Error strategies in Go

Reading a missing key from a Go map gives the zero value, so `settings[key] == ""` can't tell a missing key from an empty value. The two-value form can: `v, ok := settings[key]`. Then validate `v` and return the error; don't turn it into the default.

For several errors at once, Go 1.20 added `errors.Join`. Its `Error()` is every message, one per line, `errors.Is` and `errors.As` see each part, and it returns `nil` when every argument is `nil`, so collecting checks and joining them needs no extra `if`:

```go
var problems []error
if len(name) < 3 {
	problems = append(problems, errors.New("name: too short"))
}
if !strings.Contains(email, "@") {
	problems = append(problems, errors.New("email: missing @"))
}
if err := errors.Join(problems...); err != nil {
	return err
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `v, ok := m[k]` is `Object.hasOwn` and the read in one step; a plain `m[k]` never fails, it gives `""` or `0`.
- `errors.Join` is what `AggregateError` would be if its message listed the parts.
- `strconv.Atoi("+5")` is `5`; check the characters yourself if a sign isn't allowed.
<!-- /from -->

<!-- from:default -->
## Key points
- `v, ok := m[key]` separates a missing key from a zero value.
- `errors.Join(errs...)` combines errors and is `nil` when there are none.
- Return the parse error with context; don't swap it for the default.
<!-- /from -->

## Common mistake
Using the zero value as "missing":

```go
if settings[key] == "" {
	return fallback, nil // an empty value in the config silently becomes the fallback
}
```

## In these exercises
`IntSetting` uses `v, ok :=` for the missing key, checks the characters, then calls `strconv.Atoi`. `ValidateSignup` appends one error per problem and returns `errors.Join(problems...)`.
