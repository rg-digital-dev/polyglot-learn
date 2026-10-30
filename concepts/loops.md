# Loops

Loops repeat work over a range or a collection. Languages offer some mix of three shapes:

- **Counting loops:** `for (let i = 1; i < n; i++)`, Lua's `for i = 1, n - 1`, Go's `for i := 1; i < n; i++`.
- **Iterating over a collection:** `for x of list`, `for _, x := range list`, `for x in list.iter()`, `foreach ($list as $x)`, `for _, x in ipairs(list)`.
- **Iterator chains:** `(1..n).filter(..).sum()` in Rust, or `list.some(..)` in JS.

Watch the range boundaries:

- **Inclusive or exclusive end.** Rust's `1..n` stops before `n`, while Lua's `for i = 1, n` includes `n`.
- **Where counting starts.** Lua tables start at index 1, almost everything else at 0.

```text
sum_of_multiples([3, 5], 10)
  candidates 1..9, multiples of 3 or 5: 3, 5, 6, 9  ->  23
```

`break` leaves the innermost loop early. In this exercise it stops a number from being counted twice when more than one factor divides it (15 is a multiple of both 3 and 5).

## Common mistake

Adding a number once per matching factor. With factors `[3, 5]`, the number 15 matches both and gets counted twice. Stop checking factors as soon as one matches, or collect the numbers in a set first.

The other trap is a factor of 0. `n % 0` crashes or misbehaves in every language (a division-by-zero error, NaN, or a panic), so skip it before dividing.

## Go deeper

- Exercism's language-neutral spec: [sum-of-multiples](https://github.com/exercism/problem-specifications/tree/main/exercises/sum-of-multiples)
- `learn compare loops` shows how each track spells the three loop shapes.
