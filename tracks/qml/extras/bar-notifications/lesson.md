# Time-driven state and signals out

A notification center holds what apps sent until each one expires or is dismissed. Expiry is about time, and time is where tests get slow and flaky if the code reads the real clock or starts timers. This project keeps one rule from the clock stage: time is a property, `now`, that someone else moves forward. A change handler on it does the expiring:

```qml
import QtQuick

QtObject {
    property real now: 0
    property ListModel reminders: ListModel {}
    signal due(string text)

    function remind(text, inMs) {
        reminders.append({ text: text, at: now + inMs })
    }

    onNowChanged: {
        // Backwards: removing row i doesn't shift the rows not yet visited.
        for (let i = reminders.count - 1; i >= 0; i--) {
            if (reminders.get(i).at <= now) {
                const text = reminders.get(i).text
                reminders.remove(i)
                due(text)
            }
        }
    }
}
```

In the real bar, a `Timer` moves `now` once a second (stage 8's preview does exactly that). In a test, one assignment jumps an hour ahead.

Signals are how this component talks to the rest of the shell. `popup(nid)` tells the bar to show a bubble, and `expired(nid)` lets it hide one. The component doesn't know what listens, or whether anything does. Do-not-disturb is a decision about the signal, not about storage: every notification is still kept, but most don't announce themselves.

<!-- from:typescript -->
## Coming from TypeScript
- A property changed by a single caller is the same as passing a clock into a function, the dependency-injection way of testing time.
- Removing from an array while looping forwards skips elements; the same goes for a ListModel. Loop backwards, or collect first and remove after.
- An object keyed by app name with an array for order is the usual grouping idiom. A `Map` works too.
<!-- /from -->

<!-- from:default -->
## Key points
- Drive time through a property; react in `onNowChanged`.
- Remove from a model backwards.
- Emit signals for things others may care about; keep the decision (do-not-disturb) in one place.
<!-- /from -->

## Common mistake
Using a real `Timer` inside the component for each notification's timeout. It works in the app, but tests must then `wait()` for real seconds, and expiry depends on how busy the machine is.

## In this stage
In `Notifications.qml`, implement `notify`, `dismiss`, `clear` and `groups`, make `count` and `badge` bindings, and expire notifications when `now` changes. On Linux, the freedesktop notification spec defines the same fields: app name, summary, body, urgency and an expiry timeout.