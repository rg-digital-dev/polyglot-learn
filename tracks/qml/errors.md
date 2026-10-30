# Errors in QML

QML functions report failure the JavaScript way: `throw new Error("message")` stops the function, and the caller can catch it with `try { ... } catch (e) { ... }` and read `e.message`. If nothing catches it, the engine prints a warning that includes the file and line. A `: int` or `: string` return type only describes the success path; a thrown error is not a return value, so it is never converted. In a Quickshell config this is how a helper that parses a `Process`'s output can reject unexpected text instead of showing garbage in the bar.

```qml
import QtQuick

QtObject {
    // Parse "WIDTHxHEIGHT", e.g. "1920x1080".
    function parseSize(input: string): var {
        const match = /^(\d+)x(\d+)$/.exec(input)
        if (match === null)
            throw new Error(`expected WIDTHxHEIGHT, got "${input}"`)
        const w = Number(match[1]), h = Number(match[2])
        if (w === 0 || h === 0 || w > 16384 || h > 16384)
            throw new Error("size out of range")
        return { width: w, height: h }
    }

    function describe(input: string): string {
        try {
            const s = parseSize(input)
            return `${s.width} by ${s.height}`
        } catch (e) {
            return "invalid: " + e.message
        }
    }
}
// describe("800x600") -> "800 by 600"
// describe("big")     -> "invalid: expected WIDTHxHEIGHT, got \"big\""
```

<!-- from:typescript -->
## Coming from TypeScript
- `throw`, `try`/`catch`/`finally` and `Error` work as you know them.
- `parseInt` is just as lenient as in a browser: `parseInt("80a")` is `80` and `parseInt("+80")` is `80`. Validate the characters first.
- QML's `int` is 32-bit. Range-check the JavaScript number before returning it; a huge value returned through `: int` comes out wrapped, not as an error.
- There is no `Result` type and no checked exceptions; document which functions throw.
<!-- /from -->

<!-- from:default -->
## Key points
- Signal invalid input with `throw new Error("...")`; include a message that says what was wrong.
- Catch with `try { ... } catch (e) { ... }` and read `e.message`.
- Check the input fully before converting it to a number.
- An `int` holds values from -2147483648 to 2147483647.
<!-- /from -->

## Common mistake
Returning a sentinel instead of throwing:

```qml
if (n > 65535) return -1                          // the test sees a normal return value
if (n > 65535) throw new Error("out of range")    // fixed
```

## In this exercise
Implement `function parsePort(input: string): int` in `parse-port/ParsePort.qml`. Return the port for valid input and throw an `Error` otherwise. The untouched stub's `TODO: implement` error does not count as the expected error.
