# Iterators and pipelines

An iterator produces values one at a time. Most languages let you chain operations on it: filter, map, sort, take. So "the n most frequent words" reads as a pipeline instead of a pile of loops:

```text
text -> words -> counts -> (word, count) pairs
     -> sort by count desc, then word asc -> take n -> words
```

The spelling varies by language:

- **Lazy chains:** Rust iterator adapters (`.map().filter().take()`) do no work until `collect()` or `sum()`. The same goes for JS generators and PHP generators (`yield`).
- **Eager array methods:** JS/QML `.map().sort().slice()` and PHP's `array_map` / `usort` / `array_slice` build a new array at every step.
- **Plain loops:** Go deliberately keeps `for ... range` and `slices` helpers, and only added range-over-func iterators in 1.23. Lua's generic `for` with `pairs`/`ipairs`, or a custom iterator function, is the idiom there.

## Sorting on two keys

A comparator for "count descending, then word ascending" compares the first key, and only if that's a tie, the second:

```text
compare(a, b) = compare(b.count, a.count), or if equal, compare(a.word, b.word)
```

The languages have shortcuts for this: Rust `.then_with()`, Go `cmp.Or`, PHP `[$b[1], $a[0]] <=> [$a[1], $b[0]]`, and JS `cb - ca || ...`.

## Common mistake

Relying on map iteration order. Iterating a hash map visits keys in an unspecified order in Rust, Go and Lua, and in insertion order in JS and PHP. Either way it's not the order you want, so always sort explicitly.

## Go deeper

- [Iterator (Wikipedia)](https://en.wikipedia.org/wiki/Iterator)
- Exercism's language-neutral spec for a related exercise: [high-scores](https://github.com/exercism/problem-specifications/tree/main/exercises/high-scores)
