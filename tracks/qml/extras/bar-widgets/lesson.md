# Inline components and objects made at runtime

The config names the widgets in each section of the bar, so they can't all be written out in QML ahead of time. They are made at runtime from components, and destroyed when the config changes.

An *inline component* declares a type inside a file, `component Name: Base { ... }`, without a file of its own. It's a good fit for small variations on one base. A *required property* must be given a value when an object is made, and creation fails if it isn't. That turns "forgot to pass the source" into an error at the right place.

```qml
import QtQuick

QtObject {
    id: shelf
    property int made: 0

    component Label: QtObject {
        required property string text
        property var owner: null
        Component.onDestruction: if (owner) owner.made--
    }
    component Price: Label {
        required property real amount
        text: amount.toFixed(2) + " EUR"          // satisfies Label's required text
    }

    property Component priceComponent: Component { Price {} }

    function price(amount) {
        const p = priceComponent.createObject(shelf, { amount: amount, owner: shelf })
        made++
        return p
    }
}
```

`createObject(parent, properties)` makes an object from a `Component`, sets the initial properties first (which is how required ones are given), and returns it. A required property it doesn't get makes it return `null` with a warning. `destroy()` deletes the object a moment later, when control returns to the event loop, not immediately.

Inline components don't share the scope of the file around them: inside `Label`, the id `shelf` is unknown. Anything a widget needs, it gets as a property, like `owner` above.

<!-- from:typescript -->
## Coming from TypeScript
- An inline component is a small class declared next to the code that uses it; `component Price: Label` is `class Price extends Label`.
- `required property` is a required constructor argument, checked when the object is created.
- `destroy()` is deferred, like disposing on the next tick. Code that counts on it must wait (tests use `tryCompare`).
<!-- /from -->

<!-- from:default -->
## Key points
- `component Name: Base { }` declares a type inside a file.
- `required property` makes creation fail without that value.
- `createObject(parent, { ... })` makes objects at runtime; `destroy()` removes them later.
- Pass what an inline component needs as properties; it can't see the outer ids.
<!-- /from -->

## Common mistake
Making objects with `createObject(null)` and forgetting about them. Without a parent, nothing owns the object. It may be garbage collected while still in use, or kept forever by a stray reference. Give dynamic objects a parent, and destroy the ones you replace.

## In this stage
In `Widgets.qml`, declare `Widget` and the five widget kinds as inline components, fill in `widgetComponent` and `components`, and implement `create`, `build` and `textOf`.