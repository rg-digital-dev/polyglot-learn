# Interfaces and traits

An interface is a set of methods. Code written against the interface works with any type that has those methods, without knowing which concrete type it got. That's the point of `describe_all`: it calls `area()` and `label()` and never asks "is this a rect or a square?".

How each language decides that a type "has the methods":

- **Declared (nominal).** PHP `implements` and Rust `impl Trait for Type` say it explicitly, and the compiler checks it.
- **Structural.** A Go type satisfies an interface just by having the methods, with no `implements` keyword. TypeScript's `implements` is optional and only type-checks the shape.
- **Duck typing.** Lua and QML (JavaScript) only find out when the method is called. If the object has `area`, it works.

```text
describe_all([rect(2, 3), square(4)])
  -> ["square 4: 16", "rect 2x3: 6"]
```

Rust shows the cost most directly. A list holding different types that share a trait needs indirection, `Vec<Box<dyn Shape>>`, and every method call goes through a vtable. Generics (`T: Shape`) avoid that, but then the list can only hold one type.

## Stable sorting

Sorting "largest area first" leaves ties: `square 2` and `rect 1x4` both have area 4. A **stable** sort keeps equal items in their input order. Stability depends on the language:

| Stable | Not stable |
|---|---|
| JS `sort` (by ES2019), Rust `sort_by`, PHP `usort` (8.0+), Go `slices.SortStableFunc` | Lua `table.sort`, Go `slices.SortFunc`, QML's JS engine with more than a few items |

When stability isn't guaranteed, break ties on the original index yourself.

## Common mistake

Checking the concrete type inside `describe_all`, with `instanceof`, a type switch or `getmetatable`. If adding a `circle` would force you to edit `describe_all`, the interface isn't doing its job.

## Go deeper

- [Duck typing (Wikipedia)](https://en.wikipedia.org/wiki/Duck_typing)
- `learn compare interfaces` puts nominal, structural and duck typing side by side.
