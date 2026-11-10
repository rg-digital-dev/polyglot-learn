# Changelog

All notable changes to this project are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- *Syntax Primer*: seven short reading pages before the first exercise, on how
  each track writes files, variables and constants, types, operators and
  strings, control flow, functions, and lists, maps and tuples. Each page has a
  cross-track table and a note for your track. It's optional: no tests, and an
  experienced learner can skip it.
- `learn read [page]` shows the next unread page (or the one you name) and marks
  it read. `learn status` lists the pages, `learn compare` accepts them, and
  `learn start` and `learn status` point a new learner at the primer until they
  read a page or pass an exercise.
- Web UI: a reading page with previous/next and a track picker, the primer on
  the course page, and a "Read the Syntax Primer first" button for new learners.

### Added
- Per-track paths: `tracks/<lang>/path.toml` orders a track's stages. It uses
  shared modules from `course.toml` (and can `skip` exercises) and holds the
  track's own stages, so the "Deeper" and "in Practice" modules moved there
  from `course.toml`. Progress is unchanged.
- Track families (compiled: Rust, Go; JavaScript: TypeScript, QML; scripting:
  Python, Lua, PHP), with a one-line "what it's for" per track in `learn
  status` and on the web UI's track cards. `learn compare` and the web compare
  page start with your track's family; `--all` adds the rest, and mixing
  families shows a short note on why they differ (`families.toml`).
- `learn init` asks what you're here for: learning one language, switching
  from one you know (notes lead with their "coming from" section and the task
  card says so), or comparing tracks you pick (`learn compare` starts with
  them and `learn status` shows every track side by side). Flags: `--goal`,
  `--compare rust,go`.

- Web UI, rebuilt around your track's path:
  - The home page is your path: a "Continue" card, the primer for new
    learners, progress, and every stage with its marks.
  - A first-run screen (`/welcome`, also "Change track or goal") picks the
    track, the language you know and your goal, like `learn start` and
    `learn init`.
  - Exercises use three columns: the path on the left, then Lesson / Task /
    Compare tabs, then the tests and hints. Task puts the note beside the
    editor on very wide screens; Compare starts with your track's family.
    Below 1280px the path is a drawer; below 900px everything stacks.
  - Reading pages show the path too, and the old all-tracks grid moved to
    "All tracks" (`/tracks`).
  - The API adds `/api/path` and `POST /api/goal`.
- A full-screen terminal view: plain `learn` on a terminal opens it.
  - The same three areas as the web UI: your path, the open item with Lesson /
    Task / Compare tabs, and the tests, which tick off case by case.
  - Saving the exercise file runs the tests, like `learn watch`; `e` opens it in
    `$EDITOR` (or Zed), `c` checks, `h` shows a hint, `n` goes to the next item,
    `r` to the next unread page, `t` switches track, `/` finds an exercise, `?`
    lists the keys.
  - On first run it asks for the track, the language you know and your goal.
  - Narrow terminals hide the path (`p` shows it) and put the tests below.
  - `--no-tui` or `NO_TUI=1` prints `learn show` as before, and so does output
    that isn't a terminal. Subcommands are unchanged.

### Changed
- `course.toml` holds only the shared modules now. A 0.3 binary on this
  checkout still runs, but doesn't see the tracks' own stages.
- The runner is split into two crates: `learn-core` (`runner/core`: course,
  specs, progress, notes, checks and backends, with no terminal or web code)
  and the `learn` binary, which draws the engine's view data as the CLI and
  the web UI. Nothing changes for learners; a test pins the web API's JSON
  shape. See docs/decisions/0001-track-paths.md for where this is going.

### Fixed
- `learn serve` now listens on `[::1]` as well as `127.0.0.1`. Browsers try
  `learn.localhost` over IPv6 first, so another program on the same port there
  (a dev server, say) used to answer instead. If that port is taken on `[::1]`,
  `learn serve` now says so and asks for another `--port`.

## [0.3.1] - 2026-11-09

### Fixed
- Web UI: unsaved text in the editor is no longer lost when you switch to the
  Lesson tab, another file, another exercise or another track. A dot marks
  unsaved text, and closing or reloading the browser tab asks first.

## [0.3.0] - 2026-11-07

A Python track, five new shared modules, and Rust and TypeScript in Practice.

**Before you update:**
- The TypeScript track now needs **Node 24 or newer** (was 22.6), for `using`.
  Check with `node --version`; `learn doctor` reports an older one.
- QML stubs now annotate whole numbers as `real` instead of `int`: Qt 6.7 and
  newer cut `int` to 32 bits on every call. Files you already have keep `int`,
  which is fine for the 0.2.0 exercises. If one of your QML functions gets a
  value above 2147483647, change its `int` annotations to `real`.

### Added
- *Numbers and Division*, a shared module for every track: `wrap-index` (`%`
  with negative indexes), `split-evenly` (floor vs. truncating division),
  `round-percent` (rounding halves away from zero, without float surprises)
  and `checked-multiply` (detecting 64-bit overflow), with a concept page and
  a note per track. All values stay within ±(2^53 − 1), so TypeScript and QML
  can hold them exactly.
- *Text and Unicode*, a shared module for every track: `char-count`
  (counting code points, not bytes or UTF-16 units) and `truncate` (cutting
  text to a limit without splitting a character), with a concept page and a
  note per track. The tests use accented letters, CJK, emoji and a combining
  accent.
- *Values and References*, a shared module for every track: `make-grid` (rows
  that don't share one list) and `add-bonus` (a deep copy of a map of lists),
  with a concept page and a note per track. A new `independent` spec flag
  makes the tests write into each inner list of the result and check that
  nothing else changed, in the result or the input.
- A Python track (3.12 or newer, standard library only): a runner backend with
  its own small harness, all 20 shared exercises, a note per concept, and a
  *Deeper Python* module: `lazy-pipeline` (generators), `retry-decorator`
  (decorators), `timed-block` (context managers), `parse-commands` (`match`)
  and `vector-dunder` (special methods). `learn doctor` checks for it, and CI
  runs it on Python 3.12.
- *Rust in Practice*, a second Rust-only module: `scoped-guard` (Drop),
  `error-conversion` (From and ?), `custom-iterator`, `borrow-or-own` (Cow and
  AsRef), `id-registry` (atomics, LazyLock, OnceLock) and `channel-workers`
  (channels, Send and Sync).
- *TypeScript in Practice*: `cancellable-task` (AbortSignal),
  `disposable-resource` (`using` and DisposableStack), `branded-ids` (branded
  types and `satisfies`) and `infer-types` (conditional and template literal
  types).
- *Real-world Basics*, a shared module for every track: `error-context` (wrap
  a helper's error with the setting's key without losing the reason),
  `parse-config` (errors that name the line, counting blank and comment
  lines), `int-setting` (a missing key gets the default, a bad value is still
  an error) and `validate-signup` (report every problem at once, in the
  message). With two concept pages (`error-context`, `error-strategies`) and
  notes per track.
- *Files and Resources*, a shared module for every track except QML, starting
  with `read-file`: read a text file, skip blank and comment lines, and name
  the file when it can't be read.
- `close-everything` in Files and Resources: open a group of resources,
  read them, and close every one on every way out (`Drop`, `defer`,
  `DisposableStack`, `finally`, Lua's `<close>` and a table).
- Specs can use a provided resource module (`"resources": true`): each track
  gets an `open(name)` whose resources record being closed, and the tests
  fail when one is left open or closed twice.
- A shared module can list the tracks it's on (`tracks = [...]` in
  course.toml); status, generate, selftest and the web UI follow it, and
  `learn` refuses a course that puts an exercise on a track that can't run it.
- Specs can take files: a `path` parameter plus `files` (name -> contents)
  that `learn` writes next to the tests; each track's tests pass the full path.
- Error cases in specs can check the message: `"error_contains": [...]` lists
  text the error must mention, checked in all six tracks.
- Type-checked exercises: a track-only exercise can list `typecheck` files in
  its `exercise.toml`; `learn` runs the track's type checker on them first and
  reports the result as case 1, "types check". TypeScript uses TypeScript 7's
  `tsc`, which `learn doctor` now checks for.

### Changed
- The TypeScript track needs Node 24 or newer (was 22.6), for `using`. The
  generated `tsconfig.json` adds `ESNext.Disposable`.
- Generated QML stubs annotate spec ints as `real` instead of `int`. Qt 6.7
  and newer cut `int` values to 32 bits on every call, so 4294967296 arrived
  as 0. Existing stub files are left alone.
- Spec validation rejects int values outside ±(2^53 − 1), the range that
  TypeScript and QML numbers hold exactly.

## [0.2.0] - 2026-11-02

A readable terminal, one-command installers, and the fixes from the October review.

### Added
- A readable terminal: colour (16 ANSI colours, so it suits dark and light
  themes), lessons rendered from markdown with wrapping and clickable links,
  and a short `learn show` card. New `learn lesson` (paged when long),
  `learn show --full`, `learn status --all`, and global `--color`, `--ascii`
  and `--no-pager` flags.
- `learn check` leads with a summary and shows the first failure as
  call / expected / got, the first compiler error instead of a raw dump, and
  hides Rust backtraces and test-runner chatter (all still in `-v`).
- One-command installers: `install.sh` (Linux, macOS) and `install.ps1`
  (Windows) download the release binary, check it against SHA256SUMS, and
  fetch the matching version of the course into `./polyglot-learn`.
- Specs can promise not to modify their input (`"pure": true`); the tests
  check it in TypeScript, Go, Lua and QML (`top-scores`, `shapes`).
- TypeScript: optional editor type checking with `@types/node` (see the
  track README); the generated `tsconfig.json` matches Node 22.6+.

### Changed
- `learn doctor` runs a real QML test and names the missing Qt module and
  its package.
- Notes always show their key points, with the "coming from" section added.
- Lesson fixes: no more ready-made solutions in notes, corrected examples,
  imports in the Lua and QML `slugify` stubs.
- Install with `cargo install --locked`; the minimum Rust is now 1.88.

### Fixed
- A timed-out test no longer leaves processes running (the whole process
  tree is stopped), and the timeout message no longer claims a compile error.
- `learn serve` answers 404 for unknown exercises; the web UI fits phones.

## [0.1.0] - 2026-10-30

The first release, with prebuilt `learn` binaries for Linux, macOS and Windows.

### Added
- `learn` CLI with `init`, `doctor`, `start`, `show`, `check`, `watch`, `hint`,
  `status`, `compare`, `docs`, `generate`, `check-links` and `selftest`.
- Backends for TypeScript (Node, `node --test`), Rust (Cargo workspace), Go,
  PHP, Lua 5.4 and QML (Qt 6, `qmltestrunner`).
- Module 1, Foundations: `hello`, `leap-year`, `sum-of-multiples`,
  `word-count`, `parse-port` and `top-scores`.
- Concept lessons, plus a note for each concept on every track with
  "coming from TypeScript" sections and verified further-reading links.
- External workspaces via `--workspace` / `LEARN_WORKSPACE`.
- Module 2, Building Blocks: `order-total`, `shapes`, `make-counter`,
  `top-words`, `slugify` and `run-all`, with concept lessons and notes for
  every track.
- Spec format extensions: record, class and interface types; functions that
  return functions (call-sequence cases); helper functions in their own
  modules; concurrency exercises with timing checks (real time, or a virtual
  clock in PHP, Lua and QML).
- `learn check` shows each failing case's own failure message.
- `learn serve`: a local web UI at `http://learn.localhost:4321` (127.0.0.1 only,
  foreground only) with a course overview and progress grid, exercise pages
  with rendered lessons and live per-case results over server-sent events,
  hints, a side-by-side compare view, and an optional CodeMirror editor that
  refuses to overwrite a file changed on disk. It auto-runs tests on save like
  `learn watch`. The built UI is embedded in the binary.
- No raw panics: a closed output pipe exits quietly, and anything unexpected
  prints a short message asking for a bug report.
- Track-only exercises: modules in `course.toml` can belong to one track, with
  exercises written natively in `tracks/<lang>/extras/<slug>/`.
- Module 3, one *Deeper* module per track (28 exercises in all):
  - Rust: `word-list`, `text-slices`, `robot-commands`, `config-errors`,
    `generic-stack`, `shared-state`.
  - Go: `embedded-logger`, `pipeline`, `with-timeout`, `generic-set`,
    `wrapped-errors`.
  - TypeScript: `shape-union`, `pick-fields`, `parse-unknown`,
    `typed-emitter`, `lazy-range`.
  - PHP: `card-suits`, `money-value`, `timestamps-trait`, `read-lines`.
  - Lua: `account-classes`, `vector-ops`, `number-generator`, `error-objects`.
  - QML: `temperature-binding`, `signal-counter`, `connections`, `todo-model`.

### Fixed
- CI installs Qt 6.10 instead of Ubuntu's Qt 6.4, which was missing a module
  every QML test needs.
- The built web UI is now identical on Windows and Linux (pnpm's store folder
  names are pinned to the same length).
