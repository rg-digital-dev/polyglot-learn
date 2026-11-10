# Lists, Maps and Tuples in QML

QML has two layers here too. In function bodies you use JavaScript arrays (`[1, 2, 3]`) and plain objects (`{ alice: 3 }`), or `new Map()`. On an object, `property var list: [1, 2, 3]` holds an array, and `property var scores: ({ alice: 3 })` holds an object (the parentheses stop `{` being read as a block). A `property list<int>` is a typed list. There is no tuple: use an array or an object.

```qml
import QtQuick
import QtTest

TestCase {
    QtObject {
        id: o
        property var scores: ({ alice: 3, bob: 5 })
        property var list: [1, 2, 3]
        function run() {
            console.log(list[0], list.length, list[5])    // --> 1 3 undefined
            console.log(scores.alice, scores["bob"])      // --> 3 5
            console.log(scores.zed)                       // --> undefined
            list.push(4)
            console.log(list.length)                      // --> 4
            console.log(Object.keys(scores))              // --> [alice,bob]
            console.log("alice" in scores)                // --> true
            const m = new Map()
            m.set("a", 1)
            console.log(m.get("a"), m.get("x"), m.size)   // --> 1 undefined 1
            const [a, b] = [1, 2]
            console.log(a, b)                             // --> 1 2
        }
    }
    function test_run() { o.run() }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Arrays, objects, `Map`, destructuring and array spread (`[...xs]`) all work.
- Object spread is not supported: `{ ...scores, carl: 1 }` is a syntax error. Use `Object.assign({}, scores, { carl: 1 })`.
- There is no tuple type. An array used as a pair is just an array, and nothing checks its length.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- A missing index or key gives `undefined`, not an error.
- `len(xs)` is `xs.length`, `xs.append(4)` is `xs.push(4)`, and a dict is an object: `{ alice: 3 }`.
- Object keys that are simple names need no quotes.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Arrays start at 0: `list[0]` is the first item, and `list[3]` on three items is `undefined`.
- Arrays and objects are separate things, unlike a Lua table.
- Length is `xs.length`, not `#xs`.
<!-- /from -->

<!-- from:default -->
## Key points
- Arrays are 0-based. `xs.length` is the count and `xs.push(v)` adds to the end.
- Read an object with `obj.key` or `obj["key"]`. A missing key gives `undefined`.
- Wrap an object literal in parentheses when you bind it to a property.
- Use `new Map()` when keys are not simple names.
<!-- /from -->

## Common mistake
Reading past the end and treating `undefined` as a number. `list[5] + 1` is `NaN`, with no error.
