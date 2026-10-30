# Modules in QML

QML code is split across files in three ways. A `.qml` file whose name starts with a capital letter defines a component; other files in the same folder can use it by name, and `import "dir"` brings in another folder. A folder with a `qmldir` file is a QML module that you import by name (`import MyLib`), the way the Qt modules are imported; Quickshell adds `import qs.<path>`, which resolves from the folder that holds `shell.qml`. Plain JavaScript goes in an ECMAScript module, a `.mjs` file that marks its public functions with `export`. A QML file imports it with `import "file.mjs" as Name` and calls `Name.fn()`; one `.mjs` file imports another with standard `import { fn } from "./other.mjs"`. Older code uses plain `.js` files with the same `import ... as` line, often marked `.pragma library` so that all importers share one copy; the Qt docs discourage their `.import` syntax in favour of ECMAScript imports.

```js
// format.mjs: an ECMAScript module. Only exported names are visible outside.
function pad2(n) { return String(n).padStart(2, "0") }

export function hhmm(totalMinutes) {
    return `${pad2(Math.floor(totalMinutes / 60) % 24)}:${pad2(totalMinutes % 60)}`
}
```

```qml
// Clock.qml, in the same folder
import QtQuick
import "format.mjs" as Format

QtObject {
    function label(minutes: int): string {
        return "at " + Format.hhmm(minutes)
    }
}
// label(605) -> "at 10:05"       Format.pad2 is undefined: it is not exported
```

<!-- from:typescript -->
## Coming from TypeScript
- Inside `.mjs` files, `export` and `import { x } from "./y.mjs"` work as in TypeScript.
- A QML file can't use that syntax. `import "words.mjs" as Words` is like `import * as Words`, and the name must start with a capital letter.
- There is no npm resolution: paths are relative to the importing file.
- Components need no import at all when they are in the same folder.
<!-- /from -->

<!-- from:default -->
## Key points
- Mark a module's public functions with `export`; everything else stays private.
- Import it into QML with `import "file.mjs" as Name` and call `Name.fn()`.
- Capitalised `.qml` files in the same folder are available as components.
- A `qmldir` file turns a folder into a named QML module.
<!-- /from -->

## Common mistake
Forgetting `export`. The import succeeds, but the function is missing from the namespace and calling it throws a `TypeError`:

```js
function split(text) { /* ... */ }          // Words.split is undefined
export function split(text) { /* ... */ }   // fixed
```

## In this exercise
This exercise has two files. Implement `export function split(text)` in `slugify/words.mjs` first; its tests run first. Then implement `slugify` in `slugify/Slugify.qml`, which already has `import "words.mjs" as Words`, by calling `Words.split(title)` instead of repeating its logic.
