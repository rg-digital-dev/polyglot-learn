# Functions and strings in QML

A QML object can declare methods with `function name(param: type): returnType { ... }`. The body is JavaScript, run by Qt's own engine, and the annotations use QML type names such as `int`, `real`, `bool`, `string` and `var`. Since Qt 6.7 the engine enforces these annotations on every call and coerces values to the declared types. Strings are ordinary JavaScript strings, with template literals and the usual methods, plus a QML extra: `"%1 of %2".arg(a).arg(b)`. Quickshell configs use exactly this form for helpers such as formatting a clock or battery label.

```qml
import QtQuick

QtObject {
    property string unit: "pcs"

    // `unit` resolves to the property above.
    function label(item: string, count: int): string {
        return `${count} ${item} (${unit})`
    }

    // Arrow functions are fine inside a body, but they take no annotations.
    function initials(fullName: string): string {
        return fullName.split(" ").map(part => part.charAt(0).toUpperCase()).join("")
    }

    function progress(done: int, total: int): string {
        return "%1 of %2 done".arg(done).arg(total)
    }
}
// label("apple", 3) -> "3 apple (pcs)"    initials("ada lovelace") -> "AL"
// progress(2, 5)    -> "2 of 5 done"
```

<!-- from:typescript -->
## Coming from TypeScript
- Types are QML names, not TS ones: `int`/`real` instead of `number`, `bool` instead of `boolean`, `var` for anything else. There are no unions or generics.
- Annotations are checked at run time, not just by a compiler: a `: int` function that computes `7 / 2` returns `3`.
- `let`, `const`, template literals and arrow functions all work. There is no DOM, no `window`, and no Node APIs like `require` or `process`; `console.log` is available.
- Assigning to an undeclared name throws instead of creating a global.
- `String.prototype.arg()` is a QML addition you won't find on MDN.
<!-- /from -->

<!-- from:default -->
## Key points
- Declare methods as `function name(p: type): type { ... }` inside an object.
- Annotate parameters and return types; the engine converts values to those types.
- Build strings with template literals (`` `Hi, ${name}` ``) or `.arg()`.
- Declare every local with `let` or `const`.
<!-- /from -->

## Common mistake
Forgetting `let`/`const` on a local variable:

```qml
loud = s.toUpperCase() + "!"         // Error: Invalid write to global property "loud"
const loud = s.toUpperCase() + "!"   // fixed
```

## In this exercise
Implement `function greet(name: string): string` in `hello/Hello.qml`. The test creates the component and calls `greet(...)` on it, comparing the returned string with `compare()`.
