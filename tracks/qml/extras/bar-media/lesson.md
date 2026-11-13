# States with StateGroup

A media player is a small state machine: stopped, playing, paused. QML has states built in. Each `State` has a name, a `when` condition, and a list of changes it makes while it is active. When the condition becomes true the state switches by itself, and when the state is left its changes are undone.

`Item` has a `states` list of its own, but a plain `QtObject` doesn't. A `StateGroup` provides the same machinery for any object:

```qml
import QtQuick

QtObject {
    id: door
    property bool open: false
    property bool locked: false
    property string sign: "Welcome"            // the value when no state is active

    readonly property string status: group.state === "" ? "closed" : group.state

    property StateGroup group: StateGroup {
        id: group
        states: [
            State {
                name: "open"
                when: door.open
                PropertyChanges { door.sign: "Come in" }
            },
            State {
                name: "locked"
                when: !door.open && door.locked
                PropertyChanges { door.sign: "Closed for today" }
            }
        ]
    }
}
```

The methods of a component built like this become small. `open = true` is all a "open the door" function does, and the state, the sign and anything bound to `status` follow. With no state active the group's `state` is `""`, the default, and every property has its declared value again. That is why `sign` above says "Welcome" when the door is merely closed.

<!-- from:typescript -->
## Coming from TypeScript
- It's a finite-state machine like XState, except that states are chosen by `when` conditions over your data instead of by events.
- `PropertyChanges` is a temporary override that is undone automatically when the state ends; think of it as a style that's applied while a class is present.
- Prefer conditions that can't both be true. If two `when`s hold, the first state in the list wins.
<!-- /from -->

<!-- from:default -->
## Key points
- `StateGroup { states: [...] }` gives any object named states.
- `when` selects the state; `PropertyChanges { target.prop: value }` applies while it's active.
- Keep methods to changing data; let states derive the rest.
<!-- /from -->

## Common mistake
Setting the property a state controls from a method as well:

```js
function play() { playing = true; icon = "media-playback-start" }   // fights the states
```

The state and the method now both own `icon`, and when the state is left it restores whatever value it saved, which may not be what you expect. Let one of them own each property.

## In this stage
In `Media.qml`, add the two states to `group`, make `track`, `status`, `label` and `progress` bindings, and implement `load`, `play`, `pause`, `toggle`, `stop`, `next`, `previous`, `seek` and `advance`. On Linux the real data comes from MPRIS over D-Bus, with the same idea of a playback status, a position in microseconds and a track's metadata.