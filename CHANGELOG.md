# Changelog

All notable changes to this project are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

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
