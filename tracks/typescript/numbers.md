# Numbers in TypeScript

`number` is a 64-bit float (an IEEE 754 double), so there is no integer type. `/` always divides exactly (`7 / 2` is `3.5`); `Math.trunc` cuts toward zero and `Math.floor` rounds down. `%` is a *remainder*, not a modulo: its result takes the sign of the left operand. Integers are exact up to `Number.MAX_SAFE_INTEGER` (2^53 − 1); beyond that, use `BigInt`, written with an `n` suffix, whose `/` truncates.

```ts
console.log(Math.trunc(-7 / 2), Math.floor(-7 / 2)); // -3 -4
console.log(-1 % 5, -10 % 5);                        // -1 -0
console.log(Math.round(2.5), Math.round(-2.5));      // 3 -2
console.log(2 ** 53 + 1 === 2 ** 53);                // true
console.log(2n ** 64n, -7n / 2n);                    // 18446744073709551616n -3n
```

<!-- from:javascript -->
## Coming from JavaScript
- The same numbers: `number` and `bigint` are JavaScript's types, and TypeScript won't let you mix them in one expression (`1n + 1` is a type error).
- `Number.isSafeInteger(x)` tells you whether an integer is still exact.
<!-- /from -->

<!-- from:default -->
## Key points
- `/` gives a float; `Math.trunc` and `Math.floor` turn it into an integer two different ways.
- `%` keeps the left operand's sign, and can return `-0`.
- `Math.round` rounds halves up, toward positive infinity: `-2.5` becomes `-2`.
- Above 2^53, integers aren't exact; `BigInt` is.
<!-- /from -->

## Common mistake
Forgetting `-0`. It equals `0` under `===`, but `Object.is`, `assert.deepStrictEqual` and `1 / x` tell them apart:

```ts
const r = -10 % 5;     // -0
r === 0;               // true
Object.is(r, 0);       // false
Math.round(-0.1);      // -0 as well
```

## In these exercises
`wrapIndex` and `splitEvenly` need a floored result, not `%`'s. `roundPercent` can't use `Math.round` for negative halves, and must not return `-0`. `checkedMultiply` can compare a `BigInt` product with the 64-bit limits, or check the float product with `>= 2 ** 63`, because `9223372036854775807` is already 2^63 as a number.
