# Functions and strings

A function takes typed inputs and returns a result. Every exercise in this course is one function with a fixed signature, so the first thing to learn in a new language is how it spells that signature and how it handles text.

Three things differ a lot between languages:

- **Where types go.** Some languages put the type after the name (`name: string`), some before (`string $name`), and some don't write types at all.
- **Who owns the text.** Garbage-collected languages hand strings around freely. Rust makes you choose between a borrowed view (`&str`) and an owned buffer (`String`).
- **How you build strings.** This can be interpolation (`` `Hello, ${name}!` ``), a format macro (`format!`), concatenation (`..` in Lua, `.` in PHP) or `fmt.Sprintf`.

```text
greet("  Bob  ")  ->  "Hello, Bob!"
greet("")         ->  "Hello, world!"
```

## What "trim" means

Trimming removes whitespace from both ends and leaves inner spaces alone, so `"Mary Jane"` stays as it is. Most standard libraries have a trim function, but they disagree on edge cases: some also strip Unicode spaces and some only strip ASCII ones. The tests only use spaces, tabs and newlines, so any built-in trim will pass.

## Common mistake

Checking for an empty name *before* trimming: `"   "` isn't empty, but after trimming it is. Trim first, then decide.

## Go deeper

- Exercism's language-neutral spec for a similar exercise: [two-fer](https://github.com/exercism/problem-specifications/tree/main/exercises/two-fer)
- `learn docs` lists the docs for your track, and `learn compare functions` shows every track's note side by side.
