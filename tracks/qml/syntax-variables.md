# Variables and Constants in QML

QML has two layers. The outer layer is declarative: you declare **properties** on an object, and a property can hold a **binding**, an expression that re-evaluates when what it reads changes. The inner layer is JavaScript inside functions, where `let`, `const` and `var` work as you know them. `readonly property` is the outer layer's constant. Most exercises only need the JavaScript layer.

```qml
import QtQuick
import QtTest

TestCase {
    QtObject {
        id: o
        property int count: 5
        property string label: "n=" + count   // a binding
        readonly property int max: 3

        function run() {
            let total = 1                      // JavaScript layer
            total = 2
            const xs = [1]
            xs.push(2)                         // fine: same list
            console.log(label)                 // --> n=5
            count = 6
            console.log(label)                 // --> n=6
            label = "fixed"                    // breaks the binding for good
            count = 7
            console.log(label)                 // --> fixed
        }
    }
    function test_run() { o.run() }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Inside functions `let`, `const` and `var` behave as in TypeScript. `const` stops reassigning, not mutation.
- Reassigning a `const` is a `TypeError` when the line runs. No compile step catches it.
- `var` leaks out of an `if` block, as in TypeScript.
- Assigning to a name you never declared throws `Invalid write to global property "x"` instead of making a global.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Declare locals with `let` or `const`. Plain `x = 1` on a new name is an error, not a declaration.
- A property assignment like `count = 6` changes the object, and anything bound to `count` updates.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- `let` is your `local`, and `const` is like `<const>`.
- A missing `let` does not make a global: it throws.
<!-- /from -->

<!-- from:default -->
## Key points
- Properties belong to an object: `property int count: 5`. They are the object's state.
- A property set to an expression is a binding and stays in sync with what it reads.
- Assigning a plain value to a bound property replaces the binding for good.
- `readonly property` is set once, in its declaration. Assigning later throws a `TypeError`.
- Inside functions, prefer `let` and `const`. Avoid `var`.
<!-- /from -->

## Common mistake
Assigning to a bound property when you wanted it to keep following its inputs. After `label = "fixed"`, changing `count` no longer updates `label`.
