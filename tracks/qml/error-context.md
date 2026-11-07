# Error context in QML

QML's JavaScript throws and catches like any other JavaScript: catch the error where you know something useful, then throw a new `Error` whose message adds it. To keep the original, set `cause` on the new error: the QML engine doesn't promise to support the `new Error(message, { cause })` form, so assign the property yourself. Anything can be thrown, so check `e instanceof Error` before reading `e.message`. A QML signal handler that lets an error escape only prints a warning, so the wrapping matters most in functions other code calls.

```qml
import QtQuick

QtObject {
    function readWidth(settings: var, name: string): int {
        if (!(name in settings))
            throw new Error(name + ": missing")
        try {
            return parseSize(settings[name])
        } catch (e) {
            const wrapped = new Error(name + ": " + (e instanceof Error ? e.message : String(e)))
            wrapped.cause = e
            throw wrapped
        }
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- The same `try`/`catch`, but set `cause` as a property; don't rely on the constructor option.
- Typed QML functions (`function f(x: string): int`) check types at the boundary, but `catch (e)` is still untyped.
- Helper `.mjs` modules are plain JavaScript modules, imported with `import "numbers.mjs" as Numbers`.
<!-- /from -->

<!-- from:default -->
## Key points
- Catch, then throw a new `Error` with the context in front of the old message.
- Set `wrapped.cause = e` to keep the original.
- Keep the `try` around only the call you're wrapping.
<!-- /from -->

## Common mistake
Swallowing the error and returning a default:

```qml
try { return parseSize(v) } catch (e) { return 0 }   // a typo in the settings becomes width 0
```

## In these exercises
`Numbers.parseNumber` in `numbers.mjs` throws errors that say why. `loadSetting` finds the key, calls it inside a small `try`, and throws a new error with the key in front and the original as `cause`.

`parseConfig` needs the line number in its errors: loop with an index over `text.split("\n")` so blank lines count too, and split at `indexOf("=")`. QML in Qt 6.10 has no `Object.hasOwn`, so check for a duplicate key with `Object.prototype.hasOwnProperty.call(config, key)`.
