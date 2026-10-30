# Conditionals in QML

Inside a QML function you write standard JavaScript control flow: `if`/`else`, `switch`, the ternary `cond ? a : b`, and the `&&`, `||` and `!` operators. Truthiness follows JavaScript: `0`, `""`, `NaN`, `null` and `undefined` are false, everything else is true. Use `===` and `!==` so no type conversion happens behind your back. Conditionals also show up directly in property bindings, for example a Quickshell bar that picks an icon with `text: charging ? "AC" : "BAT"`.

```qml
import QtQuick

QtObject {
    function batteryIcon(percent: int, charging: bool): string {
        if (charging)
            return "charging"
        if (percent <= 10)
            return "empty"
        return percent >= 80 ? "full" : "half"
    }

    function isWeekend(day: string): bool {
        switch (day) {
        case "sat":
        case "sun":
            return true
        default:
            return false
        }
    }
}
// batteryIcon(5, false) -> "empty"   batteryIcon(85, false) -> "full"
// isWeekend("sun") -> true           -7 % 4 -> -3 (the sign follows the left operand)
```

<!-- from:typescript -->
## Coming from TypeScript
- `if`, `switch`, `? :`, `===`, `&&` and `||` behave exactly as in TypeScript.
- There is no type narrowing or exhaustiveness check, and no compile step rejects a bad type; `qmllint` and qmlls only warn.
- A `: bool` return annotation converts whatever you return to a boolean, so returning a number by mistake does not fail loudly.
- `/` is still floating-point division. To test divisibility use `%`, never compare `x / y` against a rounded value.
<!-- /from -->

<!-- from:default -->
## Key points
- Prefer `===`/`!==` over `==`/`!=`.
- `0`, `""`, `NaN`, `null` and `undefined` are false in a condition.
- `a % b === 0` tests whether `b` divides `a`.
- With a `: bool` return type, any returned value is converted to `true` or `false`.
<!-- /from -->

## Common mistake
Returning a remainder instead of a comparison. It still type-checks, because the `bool` annotation converts it, but the meaning is inverted:

```qml
function even(n: int): bool { return n % 2 }        // even(4) -> false
function even(n: int): bool { return n % 2 === 0 }  // fixed
```

## In this exercise
Implement `function leapYear(year: int): bool` in `leap-year/LeapYear.qml`. The tests use `compare(subject.leapYear(2000), true)`, so return a real boolean.
