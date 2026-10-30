# Contributing

Thanks for helping! Exercises, notes, "coming from X" sections and new languages are all welcome. Please keep everything original: link to other resources, but don't copy their text or exercises.

By contributing you agree that code is licensed under MIT and lesson text under CC BY-SA 4.0 (see [README](README.md#licence)), and you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Adding an exercise

1. Write `specs/<slug>.json`:
   ```json
   {
     "slug": "parse-port",
     "title": "Parse Port",
     "concept": "errors",
     "summary": "One or two sentences that become the stub's doc comment.",
     "function": "parse_port",
     "params": [{ "name": "input", "type": "string" }],
     "returns": "int",
     "fallible": true,
     "hints": ["Shown one at a time by `learn hint`."],
     "cases": [
       { "description": "common port", "input": ["80"], "expected": 80 },
       { "description": "plus sign is rejected", "input": ["+80"], "error": true }
     ]
   }
   ```
   - Types: `int`, `bool`, `string`, `list<T>`, `map<string,T>`. Names are snake_case; slugs and concepts are kebab-case.
   - Each case has either `expected` or `"error": true`. Error cases need `"fallible": true`.
   - **The first case must expect a value.** Stubs that throw would otherwise "pass" error cases.
   - Descriptions must be unique; they become test names. Aim for 6–10 cases including edge cases.
   - Keep values in the 64-bit integer range, and use ASCII whitespace only (trim functions disagree about Unicode).
2. Add the slug to a module in `course.toml`.

### Richer specs

Module 2 uses four optional extensions. See the specs named below for complete examples.

- **Named types** (`order-total.json`, `shapes.json`): declare `"types"` and use their names like any other type.
  ```json
  "types": [
    { "name": "line_item", "kind": "record", "fields": [{ "name": "sku", "type": "string" }] },
    { "name": "shape", "kind": "interface", "methods": [{ "name": "area", "params": [], "returns": "int" }] },
    { "name": "rect", "kind": "class", "implements": ["shape"], "fields": [{ "name": "width", "type": "int" }] }
  ]
  ```
  - A `record` is plain data, compared by value; only records may be returned.
  - A `class` has fields (in constructor order) and gets the methods of the interfaces it implements. Each backend writes method skeletons for the learner.
  - In test data, a record or class value is an object with its fields. An interface-typed value names its class: `{ "type": "rect", "width": 2 }`.
- **Functions that return functions** (`make-counter.json`): `"returns": "fn() -> int"`. Each case creates one or more values and then calls them:
  ```json
  { "description": "independent", "create": [[0, 1], [100, 10]],
    "calls": [{ "on": 0, "expected": 0 }, { "on": 1, "expected": 100 }] }
  ```
- **Helper modules** (`slugify.json`): `"helpers"` lists extra functions, each with a `module`, its own `summary`, signature and `cases`. Every backend puts them in a separate file (a Rust `mod`, a Go subpackage, a PHP namespace, a `.mjs` module…). Helper cases run before the main function's cases.
- **Concurrency** (`run-all.json`): `"concurrency": true` for a function taking `list<int>` delays and returning `list<int>`. Each stub provides a `task(ms)` helper. Rust, Go and TypeScript tests check the real elapsed time. PHP, Lua and QML have no threads, so they use a provided virtual clock and check the ticks instead.
3. Run `cargo test --manifest-path runner/Cargo.toml`. It validates every spec.
4. If the exercise introduces a new concept, add `concepts/<concept>.md`. Notes in `tracks/<lang>/<concept>.md` are optional but very welcome.
5. Write a reference solution for each track you can and run the verification suite (below). Don't commit the solutions; they would spoil the exercise.

## Writing notes

Each note should fit on one screen and contain:

- a short neutral explanation,
- a small example that is **not** the exercise's solution,
- "coming from X" blocks,
- a common mistake,
- an "In this exercise" line.

Further-reading links don't go in the markdown. They go in `tracks/<lang>/track.toml`:

```toml
[[links.errors]]
kind = "docs"        # docs | lings | exercism
title = "The Rust Book: 9.2 Recoverable Errors with Result"
url = "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html"
```

Open every URL before you add it, and point *lings links at the exact exercise folder. `learn check-links` reports dead links.

### Adding a "coming from X" section

Notes mark background-specific blocks with HTML comments. `learn` shows the block that matches the learner's background (`learn init`), or the `default` block if none matches:

```markdown
<!-- from:python -->
## Coming from Python
- ...
<!-- /from -->
```

To support a new background, add blocks like this to as many notes as you like. Notes without a block fall back to `default`. If the background isn't offered by `learn init` yet, add it to `BACKGROUNDS` in `runner/src/notes.rs`.

## Adding a language

A track is one Rust module plus one folder. Checklist:

- [ ] `runner/src/backends/<lang>.rs` implements the `Backend` trait:
  - [ ] **Naming**: convert names with `crate::naming` (`camel`, `pascal`, `snake`, `flat`).
  - [ ] **Types and literals**: `type_name` and `literal` for every spec type, including empty lists and maps. Add cases to `backends/tests.rs`.
  - [ ] **Stub**: the summary as a doc comment, and a body that fails with `stub_message(name)` (`TODO: implement <name>`).
  - [ ] **Tests**: one test per case, named with `naming::case_name` so results can be mapped back. Tests must not accept the stub's own error as "the expected error".
  - [ ] **Errors**: use the language's own error convention.
  - [ ] **Shared files**: manifests and harness files, with no third-party dependencies if at all possible.
  - [ ] **Command and parsing**: `command` to run one exercise, and `parse_results` to read per-case pass/fail from its output.
  - [ ] **Toolchain**: a `Tool` with PATH names *and* common absolute install paths, plus `check_tool` for minimum versions.
- [ ] Register it in `BACKENDS` in `runner/src/backends/mod.rs`.
- [ ] `tracks/<lang>/README.md`: install per OS, how tests run, editor setup, references.
- [ ] `tracks/<lang>/track.toml`: `[install]` lines for `windows`, `macos`, `arch`, `debian` and `fedora`, plus links for each concept.
- [ ] One note per concept in `tracks/<lang>/`.
- [ ] Toolchain setup in `.github/workflows/ci.yml`.
- [ ] The verification suite passes for the new track.

## Working on the web UI

The UI is a Nuxt app in `runner/web`, built as static files into `runner/web/dist`. That folder is **committed**, and the `learn` binary embeds it with `rust-embed`, so learners never need Node.

```bash
learn serve --port 4321          # the API (in one terminal)
cd runner/web && pnpm install
pnpm dev                         # the UI with hot reload; /api goes to port 4321
pnpm build                       # writes runner/web/dist; commit it with your change
```

- **The server owns all logic.** It's in `runner/src/serve.rs` and calls the same `check`, `backends` and progress code as the CLI. The UI only displays it.
- **Style comes from RG Digital's design tokens** (copied unchanged into `app/assets/brand/`). Use its variables and utility classes (`.btn`, `.chip`, `.panel`, `.tag`…), and keep both light and dark readable.
- **Icons are Lucide**, via `unplugin-icons` (`import IconX from '~icons/lucide/x'`). They're bundled, so there are no network requests.
- **Every state-changing request sends `x-learn: 1`.** The server rejects requests without it, as well as requests for any host other than localhost.

## Verification suite

```bash
cargo test --manifest-path runner/Cargo.toml          # parser, naming, literals, notes
learn selftest                                        # every untouched stub must fail
learn selftest --solutions ../my-reference-solutions  # ...and every reference solution must pass
learn check-links                                     # every link still resolves
```

Reference solutions use the same layout as the workspace: `<dir>/<lang>/<stub path>`, for example `rust/hello/src/lib.rs` or `lua/leap-year/leap_year.lua`. `selftest` generates each track in a temporary folder, so your own workspace isn't touched. Tracks whose toolchain isn't installed are reported as skipped; pass `--require-all` to make that an error.
