# Enums, change handlers and signals

A QML file can declare its own enumeration. Code inside and outside the file reaches the values through the type's name, the file name without `.qml`. The values are integers, so an `int` property holds them.

Every property also gets a change signal for free: `percentChanged()` for `percent`. Its handler is written `onPercentChanged`. A component can declare signals of its own, and emitting one is a function call. Bindings answer "what is the value now?". Change handlers and signals answer "what just happened?", which is what a one-time warning needs.

```qml
import QtQuick

QtObject {
    id: tank
    enum Kind { Water, Fuel }

    property int kind: Tank.Water
    property real litres: 100
    readonly property bool empty: litres <= 0

    signal ranDry(string what)

    onEmptyChanged: {
        if (empty)
            ranDry(kind === Tank.Fuel ? "fuel" : "water")
    }
}
```

```qml
Tank {
    kind: Tank.Fuel
    onRanDry: what => console.log("out of", what)
    Component.onCompleted: litres = 0        // logs "out of fuel"
}
```

A change handler runs only when the value really changes. Setting `litres` to 0 twice emits nothing the second time, because `empty` was already true. That helps, but it isn't enough for a battery: its level wobbles around the threshold (20, 19, 21, 20), and nobody wants four warnings. The usual fix is *hysteresis*: once warned, stay quiet until the value has clearly recovered.

<!-- from:typescript -->
## Coming from TypeScript
- `enum Kind { Water, Fuel }` is a numeric enum: `Tank.Water` is 0. There are no string enums, and no reverse lookup from a number to a name.
- `signal ranDry(string what)` declares an event; `ranDry("fuel")` emits it to every handler and to every function connected with `tank.ranDry.connect(fn)`, like an `EventEmitter`.
- `onEmptyChanged` is a handler for a signal the engine emits for you, like a setter that notifies.
<!-- /from -->

<!-- from:default -->
## Key points
- `enum` in a QML file; refer to the values as `TypeName.Value`.
- `on<Property>Changed` reacts to a property changing; `signal` declares your own events.
- Use a flag in a property for "already warned", and reset it on a clear recovery.
<!-- /from -->

## Common mistake
Writing an enum value without the type name:

```qml
property int status: Charging          // ReferenceError: Charging is not defined
property int status: Battery.Charging  // right, also inside Battery.qml itself
```

## In this stage
In `Battery.qml`, write `level`, `low`, `icon`, `label` and `remaining` as bindings. Emit `lowBattery` from change handlers, and implement `parseUevent`. The `Status` enum is declared for you. On Linux, `cat /sys/class/power_supply/BAT0/uevent` shows the real input.