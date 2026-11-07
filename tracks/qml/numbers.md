# Numbers in QML

QML's JavaScript has only one number type, a 64-bit float, so integers are exact up to 2^53 − 1, and there is no `BigInt`. `/` divides exactly, `Math.trunc` and `Math.floor` turn the result into an integer two ways, and `%` takes the sign of the left operand. `Math.round` rounds halves toward positive infinity. On top of that, QML type annotations coerce values on every call (Qt 6.7 and newer): `int` is a **32-bit** integer, so the generated stubs use `real` for whole numbers.

```qml
import QtQuick

QtObject {
    function asInt(x: int): int { return x }
    function asReal(x: real): real { return x }

    Component.onCompleted: {
        console.log(Math.trunc(-7 / 2), Math.floor(-7 / 2), -1 % 5) // -3 -4 -1
        console.log(Math.round(2.5), Math.round(-2.5))             // 3 -2
        console.log(asInt(4294967296), asReal(4294967296))         // 0 4294967296
        console.log(asInt(7 / 2))                                  // 3
    }
}
```

<!-- from:typescript -->
## Coming from TypeScript
- The same `number`, `%` and `Math` as in TypeScript, but no `BigInt`: `10n` is a syntax error.
- A `: int` annotation isn't a type hint: it cuts the value to 32 bits, like `x | 0`.
- `compare(-0, 0)` passes in Qt Test, unlike `assert.deepStrictEqual`.
<!-- /from -->

<!-- from:default -->
## Key points
- One number type, exact for integers up to 2^53 − 1.
- `%` keeps the left operand's sign; `Math.floor` floors.
- `Math.round` rounds halves up: `-2.5` becomes `-2`.
- Annotate whole numbers as `real`, not `int`, unless they fit in 32 bits.
<!-- /from -->

## Common mistake
Annotating a large value as `int`:

```qml
function bytes(megabytes: int): int { return megabytes * 1048576 }
// bytes(4096) is 0: 4294967296 cut to 32 bits
```

## In these exercises
`wrapIndex` and `splitEvenly` need a floored result, not `%`'s. `roundPercent` can't use `Math.round` for negative halves. `checkedMultiply` has no `BigInt` to lean on: compare the float product with `2 ** 63`, which a double holds exactly, unlike 2^63 − 1.
