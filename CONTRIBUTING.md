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
   - A `path` parameter takes a file name (`"notes.txt"`) and the tests pass the full path to it. List the files' contents in `"files": { "notes.txt": "..." }`; `learn` writes them next to the tests. A name that isn't in `files` is a missing file. QML can't read files, so put such an exercise in a module with a `tracks` list.
   - `"resources": true` gives the function a provided `resource` module (`open(name)`, `read()`, `close()`; a name starting with `!` fails to open, `?` fails to read), written next to the stub. The tests wrap the call and fail if anything is left open or closed twice. The function must take one `list<string>`, return `list<string>` and be fallible, and every error case needs `error_contains`. QML has no resource module, so use a module with a `tracks` list.
   - An error case can add `"error_contains": ["port", "not a number"]`: text the error's message must contain (in every track: the `Err` string, `err.Error()`, the exception's or Lua error's message).
   - **The first case must expect a value.** Stubs that throw would otherwise "pass" error cases.
   - Descriptions must be unique; they become test names. Aim for 6–10 cases including edge cases.
   - Keep int values within ±(2^53 − 1), and use ASCII whitespace only (trim functions disagree about Unicode). TypeScript and QML numbers are doubles, exact only up to there, and the spec check enforces it. Intermediate results may go beyond it (see `checked-multiply.json`).
   - Non-ASCII text is fine where the exercise is about it (see `char-count.json`). Write invisible characters such as combining marks as `\u` escapes in the JSON, and expect code points, not graphemes: grapheme support isn't built into every track.
2. Add the slug to a module in `course.toml`. A shared module is on every track whose `tracks/<lang>/path.toml` uses it. To keep it off a track that can't run it, leave it out of that path and list the others in course.toml: `tracks = ["typescript", "rust", "go", "php", "lua"]`. A path can also leave out single exercises with `skip`.

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
- **Pure functions** (`top-scores.json`, `shapes.json`): `"pure": true` when the summary says not to modify the input. Tests then check that every list and map argument is unchanged after the call, and fail with "the input list was modified". TypeScript, Python, Go, Lua and QML need the check; Rust (`&[T]`) and PHP (arrays are copied) can't change the caller's value.
- **Independent results** (`make-grid.json`, `add-bonus.json`): `"independent": true` when the result must share no list with the input or with itself. The function must return `list<list<int>>` or `map<string,list<int>>`. After the call, the tests set the first item of each non-empty inner list to a value below everything in the result, one list at a time, and check that the rest of the result and every list or map argument are unchanged. It implies `pure`'s input check. Rust and PHP results can't share, so those tracks skip it.
3. Run `cargo test --manifest-path runner/Cargo.toml`. It validates every spec.
4. If the exercise introduces a new concept, add `concepts/<concept>.md`. Notes in `tracks/<lang>/<concept>.md` are optional but very welcome.
5. Write a reference solution for each track you can and run the verification suite (below). Don't commit the solutions; they would spoil the exercise.

## Adding a track-only exercise

Some features only exist in one language: Rust's ownership, Go's channels, QML's bindings. Those exercises are written by hand in that language and live in a stage of the track's own, in `tracks/<lang>/path.toml`:

```toml
[[stage]]
id = "deeper-rust"
title = "Deeper Rust"
exercises = ["word-list", "text-slices"]
```

Each exercise is a folder `tracks/<lang>/extras/<slug>/`:

```
exercise.toml   title, concept, summary, stubs, hints, cases
lesson.md       the lesson, with "coming from X" blocks like a track note
files/          copied to workspace/<lang>/<NN-stage>/<slug>/
```

```toml
title = "Word List"
concept = "ownership"          # also the key for "Go deeper" links in track.toml
summary = "What the learner implements, in a sentence or two."
stubs = ["src/lib.rs"]         # the learner's files, relative to files/; the first is the main one
hints = ["...", "..."]
cases = ["total_len counts characters of every word", "..."]   # in test order
```

- **Learner-owned files:** files listed in `stubs` are written only when missing. Every other file under `files/` (tests, manifests) is overwritten on every run, like generated tests.
- **Layout:** follow the layout and test command of the track's generated exercises, because `learn` runs native exercises the same way. Generate a track (`learn --workspace /tmp/ws generate --lang go`) and copy what you see: the test file name, the shared harness, and how stub errors are told apart from expected errors.
- **Test names:** name each test `cNN_<description>`. NN is the case's position in `cases`, and the description is lowercased with every run of other characters replaced by `_`, up to 48 characters, e.g. `c03_total_len_leaves_the_list_usable`. `cargo test` checks that every listed case has a test.
- **Stubs and solutions:** every case must fail on the untouched stub, and pass with your reference solution. Check both with `learn selftest --lang <lang> --solutions <dir>`.
- **Type checks (TypeScript):** `typecheck = ["<slug>.ts", "<slug>.types.ts"]` makes `learn` run `tsc` (TypeScript 7) on those files before the tests and report the result as the first case, which must be called `"types check"` and has no test of its own. Put `@ts-expect-error` lines in the `.types.ts` file for what the types must reject; the untouched stub's types should let them through, so it fails.
- **Links:** add "Go deeper" links for the new concept to `tracks/<lang>/track.toml`.

Track-only exercises are left out of `learn compare`; there's nothing to compare them with.

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

### Reading pages (the Syntax Primer)

A **reading** is a page with no exercise. List its id under a module's `readings` in `course.toml`, then add `concepts/<id>.md` (its first `# ` heading is the title) and a note in `tracks/<lang>/<id>.md` for every track the module is on. A test checks both. Reading ids can't reuse an exercise slug or concept, so the primer's start with `syntax-`; `learn read variables` also finds `syntax-variables`. Reading notes follow the rules above, without the "In this exercise" line. Keep tables to about 85 characters wide: a wider one is shown as a list in the terminal.

`learn read` shows a page and marks it read. Read marks are kept for the whole workspace, not per track, and never count towards an exercise total.

### Paths and families

Each track's `tracks/<lang>/path.toml` lists its stages in order (see `docs/decisions/0001-track-paths.md`):

```toml
[[stage]]
use = "foundations"            # a shared module from course.toml
skip = ["word-count"]          # optional: exercises this track leaves out

[[stage]]
id = "deeper-go"               # the track's own stage, exercises in tracks/go/extras/
title = "Deeper Go"
exercises = ["embedded-logger", "pipeline"]
```

A track without a `path.toml` gets every shared module that's on it, in course order. Tests check that every shared exercise is on some path and every `extras/` folder is on its track's path.

Each `track.toml` names its `family` (compiled, javascript or scripting) and has a one-line `for` that says what the language is for. `families.toml` describes each family and has a `why` note for every pair; `learn compare` and the web UI start with your track's family and show the note when families are mixed. A new language needs both.

### Adding a "coming from X" section

Notes mark background-specific blocks with HTML comments. `learn` shows the block that matches the learner's background (`learn init`), or the `default` block if none matches:

```markdown
<!-- from:python -->
## Coming from Python
- ...
<!-- /from -->
```

To support a new background, add blocks like this to as many notes as you like. Notes without a block fall back to `default`. If the background isn't offered by `learn init` yet, add it to `BACKGROUNDS` in `runner/core/src/notes.rs`.

## Adding a language

A track is one Rust module plus one folder. Checklist:

- [ ] `runner/core/src/backends/<lang>.rs` implements the `Backend` trait:
  - [ ] **Naming**: convert names with `crate::naming` (`camel`, `pascal`, `snake`, `flat`).
  - [ ] **Types and literals**: `type_name` and `literal` for every spec type, including empty lists and maps. Add cases to `backends/tests.rs`.
  - [ ] **Stub**: the summary as a doc comment, and a body that fails with `stub_message(name)` (`TODO: implement <name>`).
  - [ ] **Tests**: one test per case, named with `naming::case_name` so results can be mapped back. Tests must not accept the stub's own error as "the expected error".
  - [ ] **Errors**: use the language's own error convention.
  - [ ] **Shared files**: manifests and harness files, with no third-party dependencies if at all possible.
  - [ ] **Command and parsing**: `command` to run one exercise, and `parse_results` to read per-case pass/fail from its output.
  - [ ] **Toolchain**: a `Tool` with PATH names *and* common absolute install paths, plus `check_tool` for minimum versions.
- [ ] Register it in `BACKENDS` in `runner/core/src/backends/mod.rs`.
- [ ] `tracks/<lang>/README.md`: install per OS, how tests run, editor setup, references.
- [ ] `tracks/<lang>/track.toml`: `family`, a one-line `for`, `[install]` lines for `windows`, `macos`, `arch`, `debian` and `fedora`, plus links for each concept.
- [ ] `tracks/<lang>/path.toml`: the shared modules the track uses, in order, and its own stages.
- [ ] One note per concept in `tracks/<lang>/`.
- [ ] Toolchain setup in `.github/workflows/ci.yml`.
- [ ] The verification suite passes for the new track.

## Where an exercise's files live

In a track's workspace folder each exercise sits in a numbered stage folder, `<NN-stage>/<slug>/`, numbered by the track's `path.toml` (stages without exercises on that track don't count). `Ctx::discover` fills `Spec::stages`; use `spec.dir(track)` (a path) or `spec.rel(track)` (`/`-separated, for commands and generated files), or `ctx.exercise_dir(track, slug)`, never `<slug>` on its own. `ctx.exercise_at(track, rel)` goes the other way, from a file to its exercise. `backends::generate` moves folders from older workspaces into place. The solutions repo stays one folder per slug; `selftest` maps them.

## Working on the web UI

The UI is a Nuxt app in `runner/web`, built as static files into `runner/web/dist`. That folder is **committed**, and the `learn` binary embeds it with `rust-embed`, so learners never need Node.

```bash
learn serve --port 4321          # the API (in one terminal)
cd runner/web && pnpm install
pnpm dev                         # the UI with hot reload; /api goes to port 4321
pnpm build                       # writes runner/web/dist; commit it with your change
```

- **The server owns all logic.** It's in `runner/src/serve.rs`, and its JSON is the view data from `runner/core/src/views.rs`, which the CLI draws too. The UI only displays it; `runner/core/tests/api-shapes.json` pins the shape it relies on.
- **Pages:** `/` is your track's path (no track yet: `/welcome`, the first-run screen for track, background and goal), `/exercise/<slug>` and `/read/<id>` use the three-column learning layout (`.learn`: path, main, tests; the path becomes a drawer below 1280px and everything stacks below 900px), `/compare/<concept>` and `/tracks` (every track side by side). Shared pieces are components: `PathSidebar`, `TrackCard`, `ComparePanel`, `TestsPanel`, `ExerciseGrid`.
- **Style comes from RG Digital's design tokens** (copied unchanged into `app/assets/brand/`). Use its variables and utility classes (`.btn`, `.chip`, `.panel`, `.tag`…), and keep both light and dark readable.
- **Icons are Lucide**, via `unplugin-icons` (`import IconX from '~icons/lucide/x'`). They're bundled, so there are no network requests.
- **Every state-changing request sends `x-learn: 1`.** The server rejects requests without it, as well as requests for any host other than localhost.

## Working on the terminal view

Plain `learn` on a terminal runs `runner/src/tui/` (ratatui with crossterm):

- **`app.rs`** holds the state and what each key does. It never draws or reads the terminal, so key handling is unit-tested with a real course and a scratch workspace.
- **`draw.rs`** draws the panes from the same `views` as the web UI. Lessons, notes and failure details come from the CLI's renderers (`markdown.rs`, `view.rs`), and **`ansi.rs`** turns their colour codes into ratatui text, so there's one renderer. Its tests render screens with ratatui's `TestBackend`.
- **`mod.rs`** owns the terminal, runs checks on a thread (cases stream in as `Msg`s), polls the open exercise's folder for saves, and suspends the screen while `$EDITOR` runs.
- Layout: three columns from 110 columns wide; below that the path is hidden (`p`), and below 76 the tests go under the open item.

## Working on the language server

`learn lsp` is `runner/src/lsp.rs`, on `lsp-server` with plain JSON messages (no `lsp-types`).

- It never assumes a course at start-up: each request finds the workspace from its file (`Ctx::discover_from`), and files outside a workspace get nothing.
- Checks run on a thread, one at a time, and publish diagnostics on the exercise's main file, anchored to the signature line (`anchor`).
- Tests talk to the server over `lsp-server`'s in-memory connection: hover, code actions, hints, `window/showDocument`, a real save, and silence outside a workspace.

## Working on the Zed extension

The extension is `editors/zed`, a small Rust crate Zed loads as WebAssembly. It only finds `learn` and starts `learn lsp`. `learn zed` installs it by copying `extension.toml` and the built `extension.wasm` into Zed's `extensions/installed/polysyntax-lsp`, which Zed loads on its next start.

- `runner/build.rs` embeds it: from `LEARN_ZED_WASM` (the release workflow sets it) or a local `cargo build --release --target wasm32-wasip2` in `editors/zed`. Without either, the build carries no extension and `learn zed` says so.
- That build needs Rust from rustup with the `wasm32-wasip2` target. See [editors/zed/README.md](editors/zed/README.md) for working on it in Zed as a dev extension.

## Verification suite

```bash
cargo test --manifest-path runner/Cargo.toml          # parser, naming, literals, notes
learn selftest                                        # every untouched stub must fail
learn selftest --solutions ../my-reference-solutions  # ...and every reference solution must pass
learn check-links                                     # every link still resolves
```

Reference solutions use the same layout as the workspace: `<dir>/<lang>/<stub path>`, for example `rust/hello/src/lib.rs` or `lua/leap-year/leap_year.lua`. `selftest` generates each track in a temporary folder, so your own workspace isn't touched. Tracks whose toolchain isn't installed are reported as skipped; pass `--require-all` to make that an error.

The maintainers' reference solutions live in a private repository. CI checks them out with a read-only deploy key and runs `selftest --solutions` on every push to `main`; pull requests from forks only get the stub check.
