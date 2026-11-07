# Numbers and Division

The same arithmetic doesn't give the same answer everywhere. Three places where tracks disagree:

- **Integer division and remainder with negative numbers.** `-7 / 2` can be `-3` (truncated toward zero) or `-4` (floored). The remainder follows: with truncation it takes the sign of the dividend (`-1 % 5` is `-1`), with flooring the sign of the divisor (`-1 % 5` is `4`).
- **Rounding halves.** `round(2.5)` is `3` in some languages and `2` in others, and `round(-2.5)` can be `-3` or `-2`.
- **Overflow and precision.** When a product doesn't fit in 64 bits, a language may panic, wrap around, switch to a float, or just keep going with a bigger integer. And a JavaScript number holds integers exactly only up to 2^53 − 1.

| Track | `-7 / 2` (integer) | `-1 % 5` | Floor division | `round(-2.5)` | `2^63` as an int |
|---|---|---|---|---|---|
| TypeScript | `Math.trunc(-7 / 2)` → -3 | -1 | `Math.floor(a / b)` | `Math.round` → -2 (halves go up) | inexact: a number is a double |
| Python | `-7 // 2` → **-4** | **4** | `//`, `divmod` | `round` → -2 (halves to even) | fine: ints never overflow |
| Rust | `-7 / 2` → -3 | -1 | `div_euclid`, `rem_euclid` | `f64::round` → -3 | panics in debug, wraps in release |
| Go | `-7 / 2` → -3 | -1 | adjust by hand | `math.Round` → -3 | wraps silently |
| PHP | `intdiv(-7, 2)` → -3 | -1 | adjust by hand | `round` → -3.0 (a float) | becomes a float |
| Lua | `-7 // 2` → **-4** | **4** | `//` | no `math.round` | wraps silently |
| QML | `Math.trunc(-7 / 2)` → -3 | -1 | `Math.floor(a / b)` | `Math.round` → -2 | inexact, and no BigInt |

`/` alone gives `3.5` in TypeScript, QML, Python and Lua, and in PHP whenever the division isn't exact (PHP's `10 / 2` is the int `5`).

## Floor or truncate?

Both rules agree for non-negative numbers, so code that was only ever tested with positive values often has the wrong one. Flooring is what you want for anything cyclic or bucketed: a clock, a ring buffer, a list index counted from the end, or splitting money where `-7` in two parts must still add up to `-7`. When your language truncates, either add the divisor once to a negative remainder, or compute the floored quotient and derive the remainder from it (`r = a - q * b`), so the two always fit together.

## Rounding halves

There are three common rules. *Half away from zero* (12.5 → 13, −12.5 → −13) is the one taught at school and what Rust, Go, PHP and C do. *Half to even* (12.5 → 12, 13.5 → 14), also called banker's rounding, is Python's `round` and IEEE 754's default, because it doesn't push sums upward. *Half up* (12.5 → 13, −12.5 → −12) is JavaScript's `Math.round`. Lua has none built in.

Floats make it worse: `29 / 200 * 100` is `14.499999999999998`, because 0.145 has no exact binary form, so it rounds to 14 whatever the rule. Multiplying first (`29 * 100 / 200` is exactly 14.5) or rounding with integers only avoids that.

## Overflow and the 2^53 limit

Signed 64-bit integers go from −2^63 to 2^63 − 1 (about ±9.2 × 10^18). Past that:

- Rust panics in a debug build (and so in `cargo test`) and wraps in a release build. `checked_mul`, `wrapping_mul` and `saturating_mul` say which one you mean.
- Go and Lua wrap around without a word: `4294967296 * 4294967296` is `0`. Go reports overflow in *constant* expressions at compile time, though.
- PHP turns the result into a float, which a function declared `: int` then rejects under `strict_types`.
- Python's ints have no upper limit, so nothing overflows, and a 64-bit check has to be written by hand.
- TypeScript and QML have only doubles. Integers are exact up to `Number.MAX_SAFE_INTEGER` (2^53 − 1); above that, neighbouring integers merge (`2 ** 53 + 1 === 2 ** 53`), and `9223372036854775807` is read as 2^63. TypeScript has `BigInt` (`10n`) for exact large integers; QML's engine doesn't.

The exercises keep every input and result within ±(2^53 − 1), so every track can represent them exactly, and test overflow through products that go far past 2^63.

Division has edge cases of its own: dividing by zero is an error for integers in Rust, Go, PHP (`DivisionByZeroError`) and Lua (`7 // 0`), but `Infinity` in JavaScript and `inf` for floats in Lua (`7.0 // 0`). The smallest integer divided by −1 doesn't fit either: Rust panics, PHP's `intdiv` throws, and Go and Lua return the smallest integer again.

## Common mistake

Testing only with positive numbers:

```text
-1 % 5      →  4 in Python and Lua, -1 everywhere else
-7 / 2      → -4 in Python and Lua (//), -3 everywhere else
round(-2.5) → -3 in Rust, Go and PHP; -2 in Python and JavaScript
```

## Go deeper

- `learn compare numbers` puts every track's version side by side.
- [What Every Computer Scientist Should Know About Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html) goes into why 0.145 isn't exact.
