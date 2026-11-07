# Error strategies in QML

A map in QML is a plain JavaScript object, so it inherits from `Object.prototype`: `settings["constructor"]` is a function. Newer JavaScript has `Object.hasOwn` for this, but the QML engine in Qt 6.10 doesn't (`typeof Object.hasOwn` is `"undefined"`), so use `Object.prototype.hasOwnProperty.call(settings, key)`. The falsy values are the same as in JavaScript: `parseInt(v) || fallback` falls back for `"0"`, and `Number("")` is `0`.

QML also has no `AggregateError`. Collect the messages and throw one `Error` that lists them; attach the array if callers need the parts:

```qml
function check(name: string, email: string): bool {
    const problems = []
    if (name.length < 3)
        problems.push("name: too short")
    if (email.indexOf("@") === -1)
        problems.push("email: missing @")
    if (problems.length > 0) {
        const error = new Error(problems.join("; "))
        error.problems = problems
        throw error
    }
    return true
}
```

<!-- from:typescript -->
## Coming from TypeScript
- No `Object.hasOwn` and no `AggregateError` in Qt 6.10: use `hasOwnProperty.call` and one `Error`.
- Map parameters are typed `var`, so nothing checks what's in them.
- The falsy-value traps are the same as in TypeScript.
<!-- /from -->

<!-- from:default -->
## Key points
- `Object.prototype.hasOwnProperty.call(obj, key)` for "is it there?".
- `||` falls back for `0` and `""`; test the value with a regex before `Number`.
- Throw one `Error` whose message lists every problem.
<!-- /from -->

## Common mistake
Reading the key directly:

```qml
const value = settings[key] ?? fallback   // "constructor" and "toString" are always there
```

## In these exercises
`intSetting` checks `hasOwnProperty.call`, tests the value with `/^[0-9]+$/`, and throws with the key in the message. `validateSignup` pushes each problem to an array and throws one `Error` with all of them.
