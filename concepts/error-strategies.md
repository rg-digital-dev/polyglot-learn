# Error strategies

Knowing *how* to return an error is half of it. The other half is deciding *when* something is an error, and how much to report at once. Two decisions come up in almost every real program:

- **Missing or broken?** A setting that isn't there is often fine: use a default. A setting that is there but can't be read is a mistake someone made, and a default would hide it. Ask "is it there?" and "is it valid?" as two separate questions, and catch only the errors you mean to handle.
- **Fail fast or collect?** Reading a config, stop at the first bad line: what follows may depend on it. Checking a form, report every problem at once, so the user can fix them all in one go instead of one per submit.

| Track | Is the key there? | Collect several errors into one |
|---|---|---|
| TypeScript | `Object.hasOwn(obj, key)`, `map.has(key)` | `new AggregateError(errors, message)`: the message doesn't list them |
| Python | `key in d` | `ExceptionGroup(message, errors)`: `str()` doesn't list them |
| Rust | `map.get(key)` gives an `Option` | a `Vec` of errors, or one error that joins their messages |
| Go | `v, ok := m[key]` | `errors.Join(errs...)`: one line per error, `nil` when there are none |
| PHP | `array_key_exists($key, $a)` | an array of messages, joined into one exception |
| QML | `Object.prototype.hasOwnProperty.call(obj, key)` | an array of messages, joined into one `Error` |
| Lua | `t[key] ~= nil` | a table of messages, joined with `table.concat` |

## Missing isn't the same as broken

Most languages make it easy to blur the two:

```text
value = settings[key] or fallback        // "0" or 0 may count as missing
try: return parse(value) catch: return fallback   // "eighty" quietly becomes the fallback
```

Both look tidy, and both turn a typo in the config into a program that runs with the wrong value. Check presence first, then validate what's there, and let the validation error through, with the key in it.

## Report every problem

To collect problems, check every field, add each problem to a list, and fail once at the end if the list isn't empty. The final error's *message* should still name every problem: that's what gets logged or shown. Several languages have a type for "many errors" (`AggregateError`, `ExceptionGroup`) that keeps the parts beside the message rather than in it, so a log line says `invalid signup (2 sub-exceptions)` and nothing more. Use them, but give them a message that lists the parts.

## Go deeper

- `learn compare error-strategies` puts every track's version side by side.
- Alexis King's [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) is about returning the checked value (`signup` with a numeric `age`) instead of a yes/no answer.
