# Error handling

Code that parses outside input has to say "this didn't work" somehow, and every language has its own way of saying it. This exercise is the same in every track; only the error convention changes:

| Track | Convention | Failing `parse_port("80a")` |
|---|---|---|
| Rust | return `Result<T, String>` | `Err("not a number".into())` |
| Go | return `(T, error)` | `return 0, errors.New("not a number")` |
| TypeScript | throw | `throw new Error("not a number")` |
| PHP | throw | `throw new InvalidArgumentException(...)` |
| QML | throw | `throw new Error("not a number")` |
| Lua | return `nil, err` | `return nil, "not a number"` |

**Errors as values** (Rust, Go, Lua) show up in the signature, so the caller can see that a call can fail and is expected to handle it on the spot. **Exceptions** (TypeScript, PHP, QML) skip up the call stack until something catches them. That keeps the happy path short but hides which calls can fail.

In Lua, `error()` raises much like a throw. Library functions such as `io.open` still return `nil, message` for expected failures, and that's what these tests check: if your function raises, the case fails.

## Validate, then convert

Most built-in number parsers are more lenient than this exercise allows. They accept `+80`, `-1`, ` 80`, `8e1` or `0x50`, or silently stop at the first bad character. So check the shape yourself (non-empty, digits only), then convert, then check the range. Also remember that a string of 20 digits overflows a 64-bit integer.

## Common mistake

Relying on the parser to reject bad input. In JavaScript, `Number("")` is `0` and `parseInt("80a")` is `80`; in PHP, `(int) "80a"` is `80`. Each of these turns invalid input into a valid-looking port.

## Go deeper

- The real range of TCP ports: [IANA service name and port registry](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml)
- `learn compare errors` puts all six conventions side by side.
