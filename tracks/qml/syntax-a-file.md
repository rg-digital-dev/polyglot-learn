# A File at a Glance in QML

A QML file holds one component: `import` lines at the top, then one root object with braces. The file name sets the component name, so `Hello.qml` defines `Hello`. Inside the object you write properties (`property string who: "x"`) and functions. Comments are `// …` and `/* … */`. Statements end at the end of the line, and `;` is optional. QML has no `export` keyword: a `function` declared in the component can be called from outside, which is how the tests reach yours. `console.log(x)` prints.

```qml
import QtQuick

QtObject {
    id: root                  // a name other code can use to refer to this object
    property string who: "world"

    // A comment. This function is visible to the tests.
    function greet(name: string): string {
        return `Hi, ${name}!`
    }
}
```

The `hello` stub has this shape: `import QtQuick`, then `QtObject { id: root; function greet(name: string): string { … } }` in `Hello.qml`. Replace the body of `greet` and leave the first line alone.

<!-- from:typescript -->
## Coming from TypeScript
- A file is an object tree, not a list of statements. You cannot put a bare `function` or `let` at the top level.
- There is no `export`. Every function in the root object is reachable from outside.
- The `: string` after a parameter is real QML syntax, and the engine acts on it (see the Types page).
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Indentation is only for readers. Braces `{ }` mark every block.
- `//` starts a comment, not `#`.
<!-- /from -->

<!-- from:default -->
## Key points
- One file is one component, and the file name is its name.
- `import QtQuick` brings in the types you use, such as `QtObject`.
- `id: root` gives the object a name you can use inside the file.
- Functions live inside the object, so the tests call them as `subject.greet(...)`.
<!-- /from -->

## Common mistake
Putting a function outside the root object:

```qml
function greet(name) { }   // error: "Expected a qualified name id"
```
