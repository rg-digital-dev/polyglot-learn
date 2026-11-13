# JavaScript modules in QML

Settings are plain data, and the code that checks them has nothing to do with QML. That kind of code belongs in a JavaScript file. Since Qt 5.12 QML can import ECMAScript modules (`.mjs` files with `export`), which can also be tested and reused outside QML.

```js
// units.mjs
export const FACTORS = { m: 1, km: 1000, mi: 1609.344 }

export function convert(value, from, to) {
    if (!(from in FACTORS) || !(to in FACTORS))
        throw new Error("unknown unit")
    return value * FACTORS[from] / FACTORS[to]
}
```

```qml
import QtQuick
import "units.mjs" as Units

QtObject {
    property real km: 5
    readonly property real miles: Units.convert(km, "km", "mi")   // a binding through the module
}
```

A function called inside a binding is part of the binding. When `km` changes, `Units.convert` runs again. The same goes for parsing: `readonly property var parsed: Lib.parse(text)` parses the config whenever its text changes, and nothing else needs to remember to do it.

A config loader has two jobs that are easy to get wrong. A deep merge lays the user's file over the defaults: nested objects merge key by key, and lists replace, because merging `["a"]` into `["b", "c"]` has no good answer. Validation should report every problem with the path to it, not stop at the first one, so one edit can fix them all.

<!-- from:typescript -->
## Coming from TypeScript
- `.mjs` files are standard ES modules: `export function`, `export const`, and `import` between modules. In QML they're imported as a namespace with `as Name`.
- There's no `structuredClone` in Qt's engine; `JSON.parse(JSON.stringify(x))` copies plain data.
- Object spread (`{ ...a, ...b }`) isn't supported in Qt's V4 engine, so merging is written as a loop over `Object.keys`.
<!-- /from -->

<!-- from:default -->
## Key points
- Put plain logic in a `.mjs` module and `import "file.mjs" as Name` in QML.
- Merge objects deeply, replace lists, and never change the inputs.
- Report every validation problem as "path: message"; fall back to safe defaults.
<!-- /from -->

## Common mistake
Changing the defaults while merging:

```js
function merge(defaults, overrides) {
    for (const key in overrides) defaults[key] = overrides[key]   // DEFAULTS is now changed for everyone
    return defaults
}
```

The next config parsed starts from the previous one's settings. Copy first, then merge into the copy.

## In this stage
In `config.mjs`, implement `merge`, `validate` and `parse`. In `Config.qml`, make `parsed`, `config`, `errors` and `valid` bindings and implement `get`.