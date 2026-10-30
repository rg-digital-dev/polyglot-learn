# Maps

A map (also called a dictionary, hash or associative array) stores values under keys. Counting things is the classic use: walk the input, and for each item, add 1 to its count.

Each language has its own map type:

| Track | Type in this course | "Add 1, starting from 0" |
|---|---|---|
| TypeScript | `Record<string, number>` | `c[k] = (c[k] ?? 0) + 1` |
| Rust | `HashMap<String, i64>` | `*c.entry(k).or_insert(0) += 1` |
| Go | `map[string]int` | `c[k]++` (missing keys read as 0) |
| PHP | `array` | `$c[$k] = ($c[$k] ?? 0) + 1` |
| Lua | table | `c[k] = (c[k] or 0) + 1` |
| QML | JS object | `c[k] = (c[k] || 0) + 1` |

What differs between them:

- **Missing keys.** Reading a key that isn't there gives `undefined`, `nil`, a zero value, a warning or a panic, depending on the language.
- **Iteration order.** Don't rely on it unless the language guarantees it. The tests compare maps without caring about order.

```text
word_count("Go go GO!")  ->  { "go": 3 }
```

## Splitting into words

Here a word is a run of ASCII letters and digits, and everything else separates words, including apostrophes and non-ASCII letters. You can split with a regex, use a "split where this test fails" helper, or walk the characters by hand and build the current word.

## Common mistake

Forgetting the last word. When you build words character by character, the word in progress only gets added when you reach a separator, and the text doesn't end with one. Add whatever is left after the loop.

## Go deeper

- Exercism's language-neutral spec: [word-count](https://github.com/exercism/problem-specifications/tree/main/exercises/word-count)
- How maps work underneath: [Hash table (Wikipedia)](https://en.wikipedia.org/wiki/Hash_table)
