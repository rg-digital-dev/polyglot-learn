# A clock made of bindings

This project builds the data layer of a desktop status bar in eight stages, all in one folder. It is the part of a Quickshell or Plasma bar that knows the time, the battery, the workspaces and the music, with no drawing. Each stage adds a component, and the last one composes them into a `Bar` you can preview in a window.

The first stage is the core idea of QML: a *binding*. Give a property an expression instead of a value, and QML keeps it up to date. While evaluating the expression, the engine records every property it reads. When one of them changes, it evaluates the expression again.

```qml
import QtQuick

QtObject {
    id: root
    property real celsius: 20
    property bool metric: true

    // Re-evaluated whenever celsius or metric changes.
    readonly property string reading: metric ? celsius.toFixed(1) + " °C"
                                             : (celsius * 9 / 5 + 32).toFixed(1) + " °F"

    // A binding can be a whole block; its value is what it returns.
    readonly property string feel: {
        if (celsius < 10) return "cold"
        if (celsius < 25) return "mild"
        return "hot"
    }

    property string headline: reading      // a binding to another binding

    function freeze()   { headline = "frozen" }                       // assigning breaks the binding
    function unfreeze() { headline = Qt.binding(() => root.reading) } // a new binding
}
```

Assigning a value to a property *replaces* its binding: after `freeze()`, `headline` stays "frozen" however the temperature changes. Sometimes that's what you want, as in a label the user pinned. To make the property follow again, assign `Qt.binding(function)`, which installs a new binding instead of a value.

<!-- from:typescript -->
## Coming from TypeScript
- A binding works like a computed signal or a `useMemo` whose dependency list the engine works out for you. There's no dependency array to forget.
- `readonly property` can't be assigned from outside, but its binding still updates. It is read-only to the caller, not constant.
- `new Date(ms)` and its `getHours()`, `getDay()` and other methods behave as in a browser and use local time.
<!-- /from -->

<!-- from:default -->
## Key points
- `property T name: expression` is a binding; it re-evaluates when anything it read changes.
- Assigning a value later removes the binding. `Qt.binding(() => expr)` installs a new one.
- Keep inputs (`now`) as plain properties and derive everything else with bindings.
<!-- /from -->

## Common mistake
Breaking a binding without noticing, by "initialising" a property in a function:

```qml
property string label: time
Component.onCompleted: label = time   // looks harmless, but label no longer follows time
```

Nothing warns you. The label simply stops updating. Assign only when you mean to stop following.

## In this stage
In `Clock.qml`, turn `time`, `date`, `greeting` and `label` into bindings, and implement `relative`, `override` and `follow`. Nothing reads the system clock: the bar sets `now`, so tests can show any moment. From the next stage on, the project folder keeps every file you write, and checking a stage runs the tests of every stage so far.