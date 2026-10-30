# Modules

A module is a unit of code with a boundary. Some names are public (exported) and the rest stay private. This exercise is split into two files on purpose: `split` lives in a `words` module, and `slugify` has to import it.

Each language answers three questions differently:

| Track | A module is… | Public means… | Import with… |
|---|---|---|---|
| Rust | a file declared with `mod words;` | `pub fn` | `use crate::words::split;` |
| Go | a folder (package) | a Capitalised name | `import "learn/slugify/words"` |
| TypeScript | a file | `export` | `import { split } from "./words.ts"` |
| PHP | a namespace (+ a file to load) | everything in it | `require_once` + `use function Words\split;` |
| Lua | whatever table a file returns | what you put in the table | `local words = require("words")` |
| QML | a `.mjs` ES module (or a component file) | `export` | `import "words.mjs" as Words` |

Two families stand out:

- **Compiler-resolved:** Rust, Go and TypeScript find modules by path when they build or load. A missing export is an error before anything runs.
- **Runtime-loaded:** in PHP and Lua, loading a file *runs* it. PHP needs `require_once` so a file isn't loaded twice. Lua's `require` caches the module and searches `package.path`.

```text
words.split("  Rust & Go: 2026 ")  -> ["rust", "go", "2026"]
slugify("  Rust & Go: 2026 ")      -> "rust-go-2026"
```

The tests call `split` through the module, so it must be public: they check the boundary as well as the result.

## Common mistake

Copying `split` into the main file "because the import didn't work". That makes the tests pass but skips the point of the exercise. Read the error instead: it's almost always a missing `pub`/`export`, a lowercase Go name, or a wrong path.

## Go deeper

- [Modular programming (Wikipedia)](https://en.wikipedia.org/wiki/Modular_programming)
- What a slug is: [Clean URL (Wikipedia)](https://en.wikipedia.org/wiki/Clean_URL)
