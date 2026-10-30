# Structs and records

A record groups named fields into one value: a line item has a SKU, a quantity and a unit price. Every language has a way to do this, but the ways differ in three questions:

- **Declared shape or not?** Rust and Go declare a `struct`, PHP a class, and TypeScript an `interface` describing a plain object. Lua and QML just use a table or object literal, and the shape lives in comments.
- **Value or reference?** Copying a Rust or Go struct copies its data. A PHP object, a JS object or a Lua table is a reference, so two variables can point at the same record.
- **Mutable or not?** Rust fields are immutable unless the binding is `mut`. PHP's `readonly` locks them after construction. JS objects stay mutable unless you freeze them.

```text
order_total([{sku: "tea", qty: 2, unit_cents: 150}], 0)
  -> {item_count: 2, subtotal_cents: 300, discount_cents: 0, total_cents: 300}
```

## Money in whole cents

Store money as an integer number of cents. `0.1 + 0.2` isn't `0.3` in floating point, but `10 + 20` is always `30`. A percentage discount then needs integer division, rounding down: `1999 * 15 / 100` → `299`. Multiply before you divide, or you lose precision.

## How tests compare records

The tests compare records **field by field**: two records are equal when all their fields are equal. Depending on the language, that comparison is a derived `PartialEq`, a reflection-based deep equal, `deepStrictEqual`, or the harness comparing class and properties. Return exactly the fields in the spec, with nothing extra.

## Common mistake

Dividing first: `discount_percent / 100 * subtotal` is `0` for any discount below 100 with integer division. Write `subtotal * discount_percent / 100`.

## Go deeper

- [Record (Wikipedia)](https://en.wikipedia.org/wiki/Record_(computer_science))
- `learn compare structs` shows how each track declares and builds records.
