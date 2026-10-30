# Signals and handlers

A **signal** is an event a QML object can announce. Declare it with `signal name(type param, ...)` and emit it by calling it like a function: `name(a, b)`. Code reacts with a **handler** called `on` plus the signal name with a capital first letter: `signal saved(string path)` gets `onSaved`. Every property also has a change signal for free: `property int level` emits `levelChanged` whenever its value actually changes, and `onLevelChanged` handles it. Handlers written in the object's own declaration run for that object's signals. In tests, a `SignalSpy` watches one signal and records how often it fired and with which arguments.

```qml
import QtQuick

QtObject {
    id: door

    property bool open: false
    property int openings: 0
    property string lastVisitor: ""

    signal knocked(string who, int times)

    onOpenChanged: if (open) openings++               // a property change signal
    onKnocked: (who, times) => { lastVisitor = who }  // a custom signal, with its parameters

    function knock(who: string, times: int) {
        knocked(who, times)                           // emitting is a function call
    }
}
// door.knock("Sam", 3)  -> lastVisitor "Sam"
// door.open = true      -> openings 1
// door.open = true      -> no change, no signal, openings still 1
```

```qml
// In a TestCase:
SignalSpy { id: spy; target: door; signalName: "knocked" }
// door.knock("Sam", 3)
// spy.count -> 1, spy.signalArguments[0] -> ["Sam", 3] (an arguments object)
```

<!-- from:typescript -->
## Coming from TypeScript
- A signal is like a typed event emitter built into the object: no `addEventListener`, no event object, just the declared parameters.
- Emitting is synchronous: every handler has run when the call returns.
- Assigning the same value to a property does not emit its change signal.
- Write handler parameters as a function, `onKnocked: (who, times) => ...`. Using the parameter names without declaring them is deprecated in Qt 6.
<!-- /from -->

<!-- from:default -->
## Key points
- `signal name(type a, type b)` declares; `name(x, y)` emits.
- The handler is `on` + the name with a capital first letter; property `foo` gives `onFooChanged`.
- Change signals fire only when the value really changes.
- Handlers receive the signal's arguments through a function: `onName: (a, b) => { ... }`.
<!-- /from -->

## Common mistake
Getting the handler name's case wrong. QML treats it as an unknown property and the whole file fails to load:

```qml
signal stepped(int a)
onstepped: console.log(a)        // Cannot assign to non-existent property "onstepped"
onStepped: a => console.log(a)   // fixed
```

## In this exercise
Edit `signal-counter/SignalCounter.qml`. Implement `increment()`, `decrement()` and `reset()` so that `stepped(oldValue, newValue)` is emitted for every step and `limitReached(value)` once when `count` reaches `limit`. Then add an `onCountChanged` handler that counts changes and an `onStepped` handler that appends `"old->new"` to `log`. The tests watch the signals with `SignalSpy`.
