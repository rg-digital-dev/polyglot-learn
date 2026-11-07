# Error strategies in Rust

Rust keeps "missing" and "broken" apart in the types: `map.get(key)` returns an `Option`, and `str::parse` returns a `Result`. The mistake is flattening them back together. `.ok()` turns an error into `None`, and `.unwrap_or(fallback)` then can't tell "not there" from "there but wrong". Match on the `Option` first, and let the `Result` through with `?`.

There's no standard "many errors" type. Collect the messages in a `Vec` and return one error that joins them, or return the `Vec` itself as the error type when callers need the parts:

```rust
let mut problems = Vec::new();
if name.chars().count() < 3 {
    problems.push("name: too short");
}
if !email.contains('@') {
    problems.push("email: missing @");
}
if !problems.is_empty() {
    return Err(problems.join("; "));
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `Option` and `Result` replace `undefined` and `throw`, and the compiler makes you handle both.
- `.ok()` is the Rust version of an empty `catch`: easy to write, and it hides the reason.
- `"+5".parse::<i64>()` is `Ok(5)`; check the characters yourself if a sign isn't allowed.
<!-- /from -->

<!-- from:default -->
## Key points
- `let Some(v) = map.get(key) else { return Ok(fallback) };` handles only the missing case.
- Don't `.ok()` a `Result` you mean to report.
- Join collected messages into one `Err`, or make the error type a `Vec`.
<!-- /from -->

## Common mistake
Chaining the two cases into one default:

```rust
settings.get(key).and_then(|v| v.parse().ok()).unwrap_or(fallback) // "eighty" gives the fallback
```

## In these exercises
`int_setting` uses `let … else` for the missing key, checks the characters, then parses, mapping any error to a message with the key. `validate_signup` pushes each problem to a `Vec` and returns `Err(problems.join("; "))` if it isn't empty.
