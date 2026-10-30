# Loops in QML

QML functions loop with plain JavaScript: the counting `for (let i = 0; i < n; i++)`, `for (const x of list)` over the values of an array, `while`, and `do ... while`. `break` leaves the loop and `continue` skips to the next round. Array methods like `forEach`, `filter`, `some` and `reduce` are available too. A list passed in through a `var` parameter is an ordinary JavaScript array. In a Quickshell config the same loops turn up when you post-process command output, for example adding up the lines a `Process` printed.

```qml
import QtQuick

QtObject {
    // Count values above a threshold, ignoring negative readings.
    function countAbove(values: var, threshold: int): int {
        let count = 0
        for (const v of values) {
            if (v < 0)
                continue
            if (v > threshold)
                count++
        }
        return count
    }

    // Smallest power of two that is >= n.
    function nextPowerOfTwo(n: int): int {
        let p = 1
        while (p < n)
            p *= 2
        return p
    }
}
// countAbove([4, -9, 12, 30], 10) -> 2      nextPowerOfTwo(37) -> 64
```

<!-- from:typescript -->
## Coming from TypeScript
- `for`, `for...of`, `while`, `break` and `continue` are the same as in TypeScript.
- Use `let` for the counter. `var` works but is function-scoped, and a bare `i = 0` throws, because QML forbids creating globals.
- Lists cross the boundary as `var`, so there is no `number[]` type to rely on; the values are whatever the caller passed.
- An `int` result is a 32-bit integer (up to 2147483647). Larger sums are silently wrapped when returned.
<!-- /from -->

<!-- from:default -->
## Key points
- `for (const x of list)` visits values; the classic `for (let i = 0; ...)` gives indexes.
- `continue` skips the rest of the current iteration; `break` stops the loop.
- An empty array simply runs the body zero times.
- `x % 0` is `NaN`, not an error, so skip zero divisors explicitly.
<!-- /from -->

## Common mistake
Using `for...in` on an array. It walks the indexes as strings, not the values:

```qml
for (const f in factors) total += f   // "0", "1", ... string concatenation
for (const f of factors) total += f   // fixed: the values
```

## In this exercise
Implement `function sumOfMultiples(factors: var, limit: int): int` in `sum-of-multiples/SumOfMultiples.qml`. `factors` is a JavaScript array of integers (possibly empty); count numbers strictly below `limit`.
