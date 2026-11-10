# Functions in QML

You declare a function inside an object: `function name(a: type, b: type): type { … }`. The body is JavaScript. Code in it can read and write the object's properties by name, so a function can use `base` without taking it as a parameter. Return with `return`: the last expression of a body is not returned. Default values (`p = 2`), arrow functions and destructuring all work. Types on parameters are optional, and the engine converts values to them when you write them (Qt 6.7 and later).

```qml
import QtQuick
import QtTest

TestCase {
    QtObject {
        id: o
        property int base: 10
        function addBase(x: real): real { return x + base }
        function divmod(a: int, b: int): var { return [Math.floor(a / b), a % b] }
        function pow(x, p = 2) { return x ** p }
        function noReturn(x: int): int { }
        function run() {
            console.log(addBase(1.5))               // --> 11.5
            const [q, r] = divmod(7, 2)
            console.log(q, r)                       // --> 3 1
            console.log(pow(3), pow(3, 3))          // --> 9 27
            console.log([1, 2, 3].map(x => x * 2))  // --> [2,4,6]
            console.log(noReturn(1))                // --> 0
        }
    }
    function test_run() { o.run() }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Methods are declared with `function` inside an object. Arrow functions are for use inside bodies.
- Arrow functions take no annotations.
- A `: int` return type converts, so a function that returns nothing gives `0`, not `undefined`.
- Return several values with an array and unpack with `const [q, r] = f()`.
<!-- /from -->

<!-- from:python -->
## Coming from Python
- Write `return`. A function does not return its last expression.
- The body is in braces, with no `:` after the parameters. Python's `lambda` is `x => x * 2`.
- Multiple values come back as an array: `return [q, r]`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- A function cannot return several values. Return an array and unpack it.
- Defaults go in the signature: `p = 2`.
<!-- /from -->

<!-- from:default -->
## Key points
- The signature is `function name(param: type): type`. The return type follows the colon.
- Inside the object, its properties are in scope by name.
- Always `return` the value. The tests look at what the function returns, not at what it prints.
- Return two values as an array.
<!-- /from -->

## Common mistake
Printing the answer instead of returning it: `console.log(total)` shows the right text and the test still fails. End with `return total`.
