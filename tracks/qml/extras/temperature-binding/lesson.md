# Property bindings

A QML property declared with an expression after the colon is a **binding**, not a one-time value. QML records which properties the expression reads and re-evaluates it whenever one of them changes, so derived values never go stale. Bindings can read other bindings, and the updates chain through. An ordinary JavaScript assignment to the same property is different: it stores a value and **removes the binding** for good. To put a binding back from JavaScript, assign `Qt.binding(() => expression)`. A `readonly property` must be given its value or binding in the declaration, and any later assignment throws.

```qml
import QtQuick

QtObject {
    id: box

    property real width: 4
    property real height: 3

    property real area: width * height                  // re-evaluated when width or height changes
    property string summary: `${width} x ${height} = ${area}`
    property bool square: width === height
    readonly property real diagonal: Math.sqrt(width * width + height * height)

    function freezeArea() {
        area = area                                     // an assignment: the binding is gone
    }

    function thawArea() {
        area = Qt.binding(() => width * height)        // a binding again
    }
}
// box.width = 5      -> area 15, summary "5 x 3 = 15", diagonal 5.83...
// box.freezeArea(); box.width = 10   -> area stays 15
// box.thawArea()     -> area 30
// box.diagonal = 1   -> TypeError: Cannot assign to read-only property "diagonal"
```

<!-- from:typescript -->
## Coming from TypeScript
- A binding works like a getter that is cached and recomputed for you: `get area() { return this.width * this.height }`, except that QML also emits `areaChanged` so other bindings and handlers follow.
- `obj.area = 5` in JavaScript does not "set the getter's result"; it throws the binding away. In TypeScript nothing like that can happen silently.
- `readonly property` is enforced at run time, unlike TypeScript's compile-time `readonly`.
- Arrow functions inside `Qt.binding()` are the usual way to write the expression.
<!-- /from -->

<!-- from:default -->
## Key points
- `property T name: expression` is a live binding; `name = value` in a function replaces it with a fixed value.
- `name = Qt.binding(() => expression)` restores a binding from JavaScript.
- Bindings can depend on other bindings; the updates chain.
- `readonly property` needs its value in the declaration and throws on assignment.
<!-- /from -->

## Common mistake
Assigning a "reset" value in a function and expecting the property to keep following its inputs:

```qml
function refresh() { area = width * height }                  // computed once, then frozen
function refresh() { area = Qt.binding(() => width * height) } // fixed: a binding again
```

No error is reported. Run with `QT_LOGGING_RULES="qt.qml.binding.removal.info=true"` to see `qt.qml.binding.removal: Overwriting binding on ...` whenever an assignment replaces a binding.

## In this exercise
Edit `temperature-binding/TemperatureBinding.qml`. Give `fahrenheit`, `label`, `freezing` and `kelvin` bindings on `celsius` (keep `kelvin` readonly), then implement `pin(value)`, which assigns `fahrenheit` and so breaks its binding, and `unpin()`, which restores it with `Qt.binding()`. The tests change `celsius` and check that every derived value follows.
