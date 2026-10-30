# Connecting objects

A handler like `onClicked` can only be written where the object that owns the signal is declared. To react to **another** object's signals, use a `Connections` element: set its `target` to that object and write one function per signal, named like the handler (`function onReadingChanged() { ... }`). When `target` changes, `Connections` moves to the new object, and while it is `null` it does nothing. From JavaScript you can also connect at run time: `obj.someSignal.connect(fn)` calls `fn` on every emission until `obj.someSignal.disconnect(fn)`. `fn` can itself be a signal, which forwards the emission with the same arguments. Each `.qml` file in a folder is a component type named after the file (`Lamp.qml` gives `Lamp`), so the file name must start with a capital letter.

```qml
// Switch.qml
import QtQuick

QtObject {
    property bool pressed: false
    signal broken(string why)
}
```

```qml
// Lamp.qml
import QtQuick

Item {                                   // an Item, so it can contain Connections
    id: lamp

    property Switch control: null
    property bool lit: false
    signal fault(string why)

    Connections {
        target: lamp.control             // follows the property; null is fine
        function onPressedChanged() { lamp.lit = lamp.control.pressed }
        function onBroken(why) { lamp.fault(why) }
    }

    function watchSpare(spare: Switch) {
        spare.broken.connect(lamp.fault)     // forward one signal to another
    }

    function dropSpare(spare: Switch) {
        spare.broken.disconnect(lamp.fault)  // must name the same target
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `Connections` is a declarative `addEventListener` whose target can change; the old listener is removed for you.
- `signal.connect(fn)` / `signal.disconnect(fn)` are the imperative version, like `on`/`off` on an emitter. `disconnect` needs the same function object, so don't pass a fresh arrow function to both.
- A `QtObject` has no child list, so it can't contain a `Connections` element. Use an `Item` (or keep the element in a `property`).
<!-- /from -->

<!-- from:default -->
## Key points
- `Connections { target: obj; function onSomething(args) { ... } }` handles another object's signals.
- `Connections` follows its `target` property and ignores `null`.
- `a.sig.connect(b.sig)` forwards a signal; `disconnect` with the same target undoes it.
- Every `PascalCase.qml` file next to yours is a type you can use by name.
<!-- /from -->

## Common mistake
Writing the handler in the Qt 5 style, as a property. It still runs but warns, and the parameters are not declared:

```qml
Connections { target: sensor; onAlarm: console.log(reason) }
// Parameter "reason" is not declared. Injection of parameters into signal handlers is deprecated.
Connections { target: sensor; function onAlarm(reason) { console.log(reason) } }   // fixed
```

## In this exercise
Edit `connections/Thermostat.qml`; `Sensor.qml` is provided. Add a `Connections` element on `root.sensor` that sets `heating` when the reading changes and forwards `alarm(reason)` as `warning(reason)`, re-check `heating` when `target` changes, and implement `listen(other)` and `ignore(other)` with `connect` and `disconnect`. The tests replace the sensor and expect the thermostat to follow it.
