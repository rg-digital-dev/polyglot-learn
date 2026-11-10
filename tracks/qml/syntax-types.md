# Types and Annotations in QML

QML types come from the engine, not from TypeScript. A property has a declared type (`property int n: 1`), and the engine converts or rejects what you assign. Since Qt 6.7 the same holds for typed functions: `function add(a: int, b: int): int` converts its arguments and its result. So annotations are enforced at run time, usually by converting the value without an error. Use `var` for "anything", which is plain JavaScript. Some course stubs write whole-number parameters as `real` to keep precision, so read the signature you are given.

```qml
import QtQuick
import QtTest

TestCase {
    QtObject {
        id: o
        property int n: 1
        property var any: 3
        function add(a: int, b: int): int { return a + b }
        function addU(a, b) { return a + b }   // untyped: plain JavaScript
        function run() {
            console.log(add(2.7, 0))           // --> 2
            console.log(add("abc", 1))         // --> 1 (the "abc" became 0)
            console.log(addU("2", 3))          // --> 23
            n = "7"
            console.log(n, typeof n)           // --> 7 number
            n = 2.9
            console.log(n)                     // --> 2
            try { n = "abc" } catch (e) { console.log(e) }  // --> Error: Cannot assign QString to int
            console.log(typeof 1, typeof 1.5)  // --> number number
        }
    }
    function test_run() { o.run() }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Names differ: `int` and `real` instead of `number`, `bool` instead of `boolean`, `var` for any value. No unions, no generics.
- The annotation changes the value. `add(2.7, 0)` returns `2` and `add("abc", 1)` returns `1`, silently.
- Nothing like `tsc` runs. `qmllint` can warn, but it is not part of running the code.
- `typeof` reports `"number"` for whole and decimal values alike. Without annotations, `"2" + 3` is `"23"`.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Annotations are not hints here. They convert values and sometimes throw.
- An `int` property set to `2.9` holds `2`. Set to `"abc"` it throws.
<!-- /from -->

<!-- from:default -->
## Key points
- Basic types: `int`, `real`, `string`, `bool`, `var` (anything).
- A typed property or parameter converts what it receives. A value that cannot convert can throw or become `0`.
- An `int` property set to `"7"` holds the number 7. Set to `"abc"` it throws.
- Untyped functions and `var` behave like plain JavaScript.
<!-- /from -->

## Common mistake
Trusting a `: int` parameter to reject bad input. It converts instead, so `"abc"` arrives as `0` with no error.
