# Interfaces in QML

At the QML level a reusable type is a component, a `.qml` file or an inline `component`, and there is no `class` member you can add to a QML object. For data with behaviour inside functions, the simple idiom is a factory function: a function that returns an object literal holding both the fields and the methods. An "interface" is then just a set of method names that every such object provides. Code that receives the objects calls those methods and never asks which factory made them (duck typing). Methods written in shorthand form (`send(msg) { ... }`) can use `this`, and they can also read the factory's parameters directly, because they are closures.

```qml
import QtQuick

QtObject {
    // One factory function per "class"; each object carries its own methods.
    function email(address: string): var {
        return {
            address,
            send(msg) { return `mail to ${this.address}: ${msg}` }
        }
    }

    function sms(number: string): var {
        return {
            send(msg) { return `text to ${number}: ${msg.slice(0, 20)}` }  // number is captured
        }
    }

    // Works with anything that has send(); it never asks what it was given.
    function broadcast(notifiers: var, msg: string): var {
        return notifiers.map(n => n.send(msg))
    }
}
// broadcast([email("a@example.org"), sms("555-0100")], "build passed")
//   -> ["mail to a@example.org: build passed", "text to 555-0100: build passed"]
```

<!-- from:typescript -->
## Coming from TypeScript
- No `interface` or `implements`. A missing method shows up only when called: "Property 'send' of object [object Object] is not a function".
- A JS `class` does work inside a function body, but a factory function is the usual style in QML and fits a `var` return.
- Don't write methods as arrow functions if they use `this`; arrows take `this` from the surrounding code.
- `Array.prototype.sort` is not stable in Qt 6.10's engine: equal elements can change order even in a four-element array.
<!-- /from -->

<!-- from:default -->
## Key points
- A factory function returns an object with fields and methods.
- Code that accepts "a shape" should only call the agreed methods.
- Shorthand methods can use `this` or the factory's captured parameters.
- `sort` may reorder equal elements; break ties on the original index.
<!-- /from -->

## Common mistake
Relying on `sort` to keep ties in input order:

```qml
items.sort((a, b) => b.priority - a.priority)              // equal priorities may swap
```

QML's sort isn't stable. When two items compare equal, the comparator needs something else to decide by: where each item was in the input.

## In this exercise
In `shapes/Shapes.qml`, complete the factory functions `function rect(width: int, height: int): var` and `square`, each returning an object with `area()` and `label()`, then implement `function describeAll(shapes: var): var`. It returns strings like `"rect 2x3: 6"`, largest area first, with equal areas in input order.
