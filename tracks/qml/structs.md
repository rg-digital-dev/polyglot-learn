# Records in QML

QML has two kinds of "record". QML object types, such as a `QtObject` with `property int count`, are full objects: they can have bindings, signals and change notifications, and they are declared in `.qml` files. They are the right tool for state that the UI shows and reacts to. For plain data passed between functions, use JavaScript object literals: `{ room: "attic", celsius: 31 }`. They are cheap to create, you read them with `r.room`, and a `var` parameter or return type passes them through unchanged. Nothing checks their shape, so a misspelled field just reads as `undefined`. In a Quickshell config, parsed command output often becomes arrays of objects like these before a `Repeater` displays them.

```qml
import QtQuick

QtObject {
    // Readings are plain objects such as { room: "attic", celsius: 31 }.
    function summarize(readings: var): var {
        let total = 0
        let hottest = null
        for (const r of readings) {
            total += r.celsius
            if (hottest === null || r.celsius > hottest.celsius)
                hottest = r
        }
        const count = readings.length
        return {
            count,                              // shorthand for count: count
            average: count > 0 ? Math.floor(total / count) : 0,
            hottestRoom: hottest ? hottest.room : ""
        }
    }
}
// summarize([{ room: "attic", celsius: 31 }, { room: "cellar", celsius: 12 }])
//   -> { count: 2, average: 21, hottestRoom: "attic" }
```

<!-- from:typescript -->
## Coming from TypeScript
- There is no `interface` or `type` for object shapes in QML's JavaScript. Annotate with `var` and document the fields in a comment.
- A typo is not an error: `2 * item.unitCent` is `NaN`, and the `NaN` travels on silently.
- All numbers are doubles, so `/` gives fractions. Use `Math.floor` for division that must round down.
- Shorthand properties work, but object spread (`{ ...a }`) is a syntax error in Qt 6.10's engine. Build the new object field by field.
<!-- /from -->

<!-- from:default -->
## Key points
- Use object literals for data and QML object types for UI state.
- Read fields with `obj.field`; a missing field is `undefined`.
- `Math.floor(a / b)` is integer division for non-negative numbers.
- `compare()` checks objects field by field, ignores key order and fails on extra or missing keys.
<!-- /from -->

## Common mistake
Dividing without rounding when the result must be a whole number:

```qml
const fee = amount * 3 / 100               // 29.97 for an amount of 999
const fee = Math.floor(amount * 3 / 100)   // fixed: 29
```

## In this exercise
Implement `function orderTotal(items: var, discountPercent: int): var` in `order-total/OrderTotal.qml`. Each item is a plain object like `{ sku: "tea", qty: 2, unitCents: 150 }`. Return an object literal with the four summary fields described in the stub, all whole numbers.
