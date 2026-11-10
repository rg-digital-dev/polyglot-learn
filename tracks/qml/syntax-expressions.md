# Operators and Strings in QML

Expressions in QML are JavaScript expressions, in property bindings and in function bodies. Use `===` and `!==`: the two-character forms convert types first. Join text with `+` or, better, a template literal with backticks. QML also adds `.arg()` for numbered placeholders. Strings have the usual methods, such as `.trim()`, `.toUpperCase()` and `.split()`.

```qml
import QtQuick
import QtTest

TestCase {
    QtObject {
        id: o
        property string who: "Ann"
        function run() {
            console.log(1 == "1", 1 === "1")            // --> true false
            console.log("Score: " + 5)                  // --> Score: 5
            console.log(`Hi, ${who}!`)                  // --> Hi, Ann!
            console.log("%1 of %2".arg(1).arg(2))       // --> 1 of 2
            console.log("  hi ".trim() + "|")           // --> hi|
            console.log(0 || "d", 0 ?? "d")             // --> d 0
            console.log(7 / 2, 7 % 3, -7 % 3, 2 ** 3)   // --> 3.5 1 -1 8
        }
    }
    function test_run() { o.run() }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Almost everything is the same: `===`, `&&`, `||`, `!`, `??`, `?.` and template literals.
- `.arg()` is a QML addition to strings. It is not on MDN.
- No compile step, so `"a" - 1` gives `NaN` instead of an error.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- `and`, `or` and `not` are `&&`, `||` and `!`. `a || b` returns one of its operands, like `a or b`.
- `/` always gives a decimal: `7 / 2` is `3.5`. Use `Math.floor(7 / 2)` for `//`.
- `"Score: " + 5` works and gives `"Score: 5"`. Python raises an error.
- `0 || "d"` is `"d"`, but `0 ?? "d"` is `0`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- `..` is `+`, and `~=` is `!==`.
- `0` and `""` are false in a condition here. In Lua they are true.
- `/` is float division, as in Lua. Floor with `Math.floor(a / b)`, since there is no `//`.
<!-- /from -->

<!-- from:default -->
## Key points
- Prefer `===` and `!==`. `==` converts first.
- With a string on one side, `+` joins: `"Score: " + 5` is `"Score: 5"`.
- `a || b` and `a ?? b` give back a value, so they work as defaults. `??` only replaces `null` and `undefined`.
- `%` takes its sign from the left side: `-7 % 3` is `-1`.
<!-- /from -->

## Common mistake
Using `||` for a default when `0` or `""` is a valid value. `count || 10` replaces a real `0`. Write `count ?? 10`.
