# Composing the bar

The last stage adds no new mechanism. It assembles the seven parts into one `Bar`, and almost all of it is declarations. A part is a property of its own type holding an object, with bindings inside that connect it to the others:

```qml
import QtQuick

QtObject {
    id: house
    property real now: 0
    property alias wantedTemperature: settings.target    // an alias into a child

    property Settings settings: Settings { id: settings }
    property Thermostat thermostat: Thermostat {
        now: house.now
        target: settings.target
    }
    property Connections alarm: Connections {
        target: house.thermostat
        function onOverheated(degrees) {
            console.warn("too hot:", degrees)
        }
    }
}
```

The three tools are each other's complements. A **binding** passes a value down and keeps it current (`now: house.now`). An **alias** makes a child's property look like the parent's own, so outside code sets `house.wantedTemperature` without knowing about `settings`. **Connections** reacts to a signal from an object that is held in a property, for events rather than values. The low-battery warning becomes a notification this way, without the battery knowing notifications exist.

`Preview.qml` comes with this stage. It is a `Window` that holds a `Bar`, moves its time with a real `Timer` once a second, and shows the three sections as `Text` items. Run it with Qt's `qml` tool, which sits next to `qmltestrunner`:

```sh
qml Preview.qml
```

<!-- from:typescript -->
## Coming from TypeScript
- `Bar.qml` is the composition root, the one place that creates the parts and wires them together, like `main.ts` building an app's services.
- A binding through several objects (`cfg.config.clock.use24h`) keeps working when any link in the chain changes, which is more than a plain property path gives you in JavaScript.
- `property alias` is a getter and setter pair that forwards to another object's property, with change notifications included.
<!-- /from -->

<!-- from:default -->
## Key points
- Hold each part in a typed property; connect parts with bindings.
- `property alias` exposes a child's property as your own.
- Use `Connections` for signals between parts, so neither side imports the other.
<!-- /from -->

## Common mistake
Rebuilding derived state by hand in many places, for example calling `rebuild()` from every function that might change the config. Bind to the source instead: one `Connections` on the config's `parsedChanged` covers every way the text can change, including ones you haven't written yet.

## In this stage
In `Bar.qml`, declare the parts, make `configText` an alias and `errors`, `position`, `height`, `left`, `center` and `right` bindings, rebuild the sections when the bar is complete and when the config changes, turn a low battery into a notification, and implement `ipc` and `tick`. Then run `qml Preview.qml` in this folder to see your bar.