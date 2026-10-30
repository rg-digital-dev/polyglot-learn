# Concurrency in QML

QML's JavaScript runs on one thread, usually the GUI thread, so a long loop freezes the interface. Real asynchronous work goes through QML types that call back later. A `Timer` fires `onTriggered` after an interval. `Promise` exists for chaining callbacks. `WorkerScript` runs a JavaScript file on another thread and exchanges messages with it. In Quickshell, a `Process` starts an external command and reports its output and exit through signals. A test can't wait on those reliably, so this exercise uses a virtual clock and generators. A generator function (`function*`) pauses at each `yield`, and every call to `next()` resumes it until the next one. Several generators advanced in turn, one step each per round, behave like tasks running side by side, and the whole run takes as many ticks as the longest task.

```qml
import QtQuick

QtObject {
    // A normal function that returns a generator object.
    function download(name: string, chunks: int): var {
        return (function* () {
            for (let i = 1; i <= chunks; i++)
                yield `${name} ${i}/${chunks}`
            return `${name} done`
        })()
    }
}
// const g = download("a.iso", 2)
// g.next() -> { value: "a.iso 1/2", done: false }
// g.next() -> { value: "a.iso 2/2", done: false }
// g.next() -> { value: "a.iso done", done: true }    the return value arrives with done: true
// g.next() -> { value: undefined, done: true }
```

<!-- from:typescript -->
## Coming from TypeScript
- `async` and `await` are a syntax error in Qt 6.10's engine. `Promise` with `.then()` works.
- There is no `setTimeout`; use a `Timer` object (or `Qt.callLater` to defer a call).
- Generators work as in TypeScript, but a `function*` can't have a return type annotation. Return the generator from a normal function with a `: var` return type instead.
- Like Node's event loop, this is concurrency on one thread, not parallelism.
<!-- /from -->

<!-- from:default -->
## Key points
- `next()` returns `{ value, done }`; a task is finished when `done` is `true`.
- The value of `return` arrives with `done: true`, not with a `yield`.
- Advance every unfinished generator once per round to run them side by side.
- For real work, use `Timer`, `WorkerScript` or Quickshell's `Process`, not busy loops.
<!-- /from -->

## Common mistake
Collecting a result with `for...of` or spread. Both stop at `done` and drop the return value:

```qml
const steps = [...download("b", 2)]        // ["b 1/2", "b 2/2"]: "b done" is lost
let r = g.next(); while (!r.done) r = g.next(); r.value   // fixed: "b done"
```

## In this exercise
Implement `function runAll(delays: var): var` in `run-all/RunAll.qml`. The stub provides `property int now`, `function tick()` and `function task(ms: int): var`, which returns a generator; don't change them. The tasks should take turns on the virtual clock instead of running one after another. The tests check the results and that `now` advanced by about the largest delay, not the sum.
