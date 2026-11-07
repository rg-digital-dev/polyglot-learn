# Values and references

`b = a` looks the same in every language, but it means different things. For a number it always copies. For a list it depends on the language: either `b` gets its own copy, or `a` and `b` become two names for *one* list, so a change through one shows up through the other. Passing a list to a function, putting it in another list, or returning it works the same way as `=`.

| Track | `b = a` for a list | A real copy | Copies the inner lists too |
|---|---|---|---|
| TypeScript | shares | `[...a]`, `a.slice()` | `structuredClone(a)` |
| Python | shares | `list(a)`, `a.copy()` | `copy.deepcopy(a)` |
| Rust | *moves*: `a` can't be used any more | `a.clone()` | `a.clone()` (always deep) |
| Go | shares the backing array (slices); shares (maps) | `slices.Clone(a)` | clone each inner slice |
| PHP | copies (arrays are values); objects are shared | `$b = $a` | `$b = $a` (objects inside still shared) |
| Lua | shares (every table) | copy in a loop, or `table.move` | copy each inner table |
| QML | shares (JS arrays and objects) | `a.slice()` | copy each inner array |

Sharing is fast and often what you want. It goes wrong when the code that changes the list doesn't know someone else holds it too.

## Shallow and deep copies

Copying a list of lists, or a map of lists, with the "real copy" column gives you a new *outer* list holding the *same* inner lists. That's a **shallow copy**. Changing `copy[0][0]` still changes `original[0][0]`. A **deep copy** copies the inner lists too. Both exercises here need one: each list in the result must be its own.

Rust and PHP sit apart. Rust's ownership rules don't let two variables change one `Vec`, so `clone()` is always a full copy. PHP copies arrays on assignment (lazily, only when one side is changed), so arrays of arrays never share.

## Common mistake

Repeating one inner list instead of making new ones:

```text
row = [0, 0, 0]
grid = [row, row]     // two references to one row
grid[0][0] = 9        // grid is now [[9, 0, 0], [9, 0, 0]]
```

Shortcuts do the same: Python's `[[0] * 3] * 2`, JavaScript's `Array(2).fill([0, 0, 0])`. Make each row inside the loop instead.

## How the tests check it

After the call, the tests set the first item of each inner list in the result to a new value, one list at a time, and check that nothing else changed: not the other lists in the result, and not the input. Rust and PHP results can't share lists, so their tests skip this check.

## Go deeper

- `learn compare references` puts every track's version side by side.
- Rust's [ownership chapter](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) explains moves; the other languages share by default where Rust moves.
