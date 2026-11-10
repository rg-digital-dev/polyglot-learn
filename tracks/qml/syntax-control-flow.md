# Blocks and Control Flow in QML

Inside a function, control flow is JavaScript: braces mark blocks, and you have `if`/`else if`/`else`, `switch`, `for`, `for...of`, `while` and `?:`. At the object level there is no control flow: you express a choice with a binding such as `text: charging ? "AC" : "BAT"`.

```qml
import QtQuick
import QtTest

TestCase {
    function size(n) { return n > 5 ? "big" : "small" }

    function label(x) {
        const r = []
        switch (x) {
        case 1: r.push("one")        // no break: falls into case 2
        case 2: r.push("two"); break
        default: r.push("other")
        }
        return r.join(",")
    }

    function test_run() {
        console.log(size(9), label(1), label(2), label(9))  // --> big one,two two other
        for (let i = 0; i < 3; i++) {
            if (i === 1) continue
            console.log(i)                                  // --> 0 then 2
        }
        for (const x of ["a", "b"]) console.log(x)          // --> a then b
        let n = 0
        while (true) { n++; if (n > 2) break }
        console.log(n)                                      // --> 3
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- It is the same syntax, with the same `switch` fall through.
- No type narrowing and no exhaustiveness check: a missing case is silent.
- `for...in` gives keys as strings, even over an array.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Parentheses around the condition and braces around the block are required.
- `elif` is `else if`. `for x in xs` is `for (const x of xs)`. `for (const x in xs)` gives the indexes as strings.
- `switch` replaces `match`, but cases fall through until a `break`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- There is a `switch` and there is a `continue`.
- `for (let i = 0; i < 3; i++)` starts at 0 and stops before 3. Lua's `for i = 1, 3` includes 3.
<!-- /from -->

<!-- from:default -->
## Key points
- Blocks use braces. Indentation is only for readers.
- A `switch` case falls through to the next one until `break` or `return`.
- `for (const x of xs)` loops over values, and `for (const k in obj)` over keys.
- `break` leaves a loop, and `continue` skips to the next round.
<!-- /from -->

## Common mistake
Forgetting `break` in a `switch`: the matching case runs, then the next one too. Above, `label(1)` gives `one,two`.
