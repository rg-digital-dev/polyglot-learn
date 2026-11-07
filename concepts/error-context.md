# Error context

An error from deep inside a program usually knows *what* went wrong but not *where*: "not a number" doesn't say which setting, file or request it was about. Each layer that passes an error up can add what it knows, so the message the user finally sees reads like a path: `workers: too large (over 1000000)`. The important part is to *add* to the error rather than replace it, so the original reason (and, in most languages, the original error object) survives.

| Track | Wrap a lower-level error | Get the original back |
|---|---|---|
| Rust | `.map_err(\|e\| format!("{key}: {e}"))?`, or your own error type with `source()` | `source()`, or keep it in an enum variant |
| Go | `fmt.Errorf("%s: %w", key, err)` | `errors.Is`, `errors.As`, `errors.Unwrap` |
| TypeScript | ``new Error(`${key}: ${e.message}`, { cause: e })`` | `error.cause` |
| Python | `raise ValueError(f"{key}: {e}") from e` | `e.__cause__` |
| PHP | `new InvalidArgumentException("$key: ...", 0, $e)` | `$e->getPrevious()` |
| QML | `const w = new Error(key + ": " + e.message); w.cause = e` | `error.cause` |
| Lua | `return nil, key .. ": " .. err` | keep a table error with a `cause` field |

Everywhere, the message grows from the outside in: the outermost layer's context comes first, the root cause last.

## How much to add

Add what the caller *doesn't* already know. The function that parses one value doesn't know the key; the function that reads a setting does, so that's where the key goes. Don't repeat the cause's text in your own words, and don't log the error and pass it up as well: it then appears twice.

## Say where

When the input is text a person wrote, the most useful context is often a position: `line 4: missing '='` gets them to the problem straight away. Count lines from 1, the way editors do, and count *every* line, blank ones and comments too. If you count only the lines you keep, one blank line near the top shifts every number after it. Some splitting functions drop empty lines for you, such as Lua's `gmatch("[^\n]+")` and PHP's `preg_split` with `PREG_SPLIT_NO_EMPTY`, so number the lines before you filter them.

## Common mistake

Throwing away the original error and writing a new message from scratch:

```text
catch: throw new Error("invalid config")   // which key? why?
```

The message is shorter, but both the key and the reason are lost. Wrap instead.

## Go deeper

- `learn compare error-context` puts every track's version side by side.
- Go's [Working with Errors in Go 1.13](https://go.dev/blog/go1.13-errors) explains wrapping with `%w`; the ideas carry over to the other languages.
