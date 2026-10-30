# Closures in QML

QML functions are JavaScript functions, and they close over variables the same way. A function created inside another function keeps access to the outer function's local variables, by reference, for as long as it exists. Each call of the outer function creates new locals, so each returned function has its own private state. Closures are everywhere in QML code: arrow functions passed to `map` and `sort`, handlers connected to signals, and callbacks for a `Timer` or a Quickshell `Process`. A closure created inside a component can also see that component's properties and ids. Those belong to the component instance, though, so every closure made by that instance shares them.

```qml
import QtQuick

QtObject {
    function makeTagger(prefix: string): var {
        let n = 0                       // a fresh n for every call to makeTagger
        return name => `${prefix}-${String(++n).padStart(3, "0")} ${name}`
    }

    // Two closures from one call share the same variable.
    function makeBox(value: var): var {
        return { get: () => value, set: v => { value = v } }
    }
}
// const bug = makeTagger("BUG"), feat = makeTagger("FEAT")
// bug("crash on save") -> "BUG-001 crash on save"
// bug("typo")          -> "BUG-002 typo"
// feat("dark mode")    -> "FEAT-001 dark mode"
// const box = makeBox(1); box.set(5); box.get() -> 5
```

<!-- from:typescript -->
## Coming from TypeScript
- Closures, arrow functions and `let`/`const` scoping work as you know them.
- `for (let i = 0; ...)` gives each iteration its own `i`, so callbacks created in the loop see different values.
- The return type of a function-returning function is `var`; QML has no function type annotation.
- Component properties are not private state: they belong to the whole component instance.
<!-- /from -->

<!-- from:default -->
## Key points
- An inner function can read and change the outer function's locals.
- Each call of the outer function makes a new set of locals.
- Keep per-closure state in a `let` inside the factory, not in a component `property`.
- Return the inner function itself, not the result of calling it.
<!-- /from -->

## Common mistake
Keeping the state in a component property, so every function shares one value:

```qml
property int current: 0
function makeTicker(): var { return () => ++current }               // all tickers share it
function makeTicker(): var { let current = 0; return () => ++current }   // fixed
```

## In this exercise
Implement `function makeCounter(start: int, step: int): var` in `make-counter/MakeCounter.qml`. Return a function; each call returns the current value, beginning with `start`, and then advances it by `step`. The tests create one or two counters and call each several times, so the counters must not share state.
