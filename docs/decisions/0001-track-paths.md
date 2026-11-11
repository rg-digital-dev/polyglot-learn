# 1. Per-track paths, one engine, three interfaces

Date: 2026-11-10. Status: accepted.

## Context

Up to 0.3.x the course is concept first, with every track in lockstep: one `course.toml` lists shared modules, each exercise is one neutral spec, and the web UI's course page is a grid of exercises against seven tracks. That has three costs:

- Exercises drift to the lowest common denominator: a function from values to values that makes sense in QML, Lua and Rust at once.
- A seven-column grid suggests the languages are interchangeable, which invites ranking them. They aren't: QML describes user interfaces, Lua is embedded in a host, PHP answers one web request, Go and Rust build tools and services.
- What each language is really for lives in add-on modules at the end.

The CLI and the web server also build the same information twice (`view.rs` and `serve.rs`), so every feature is written once per interface.

## Decision

**Course**
- Each track gets a path, `tracks/<lang>/path.toml`, that orders its stages. It uses shared modules from `course.toml` (which becomes a library) and adds the track's own stages. "Deeper X" modules become a stage on their track's path, unchanged.
- Each path ends with a substantial project in the language's real use case, 6–8 stages that grow one codebase, standard library only, no network: Rust a log-analysis CLI, Go a URL-shortener service, TypeScript a typed task-tracker library, Python a data pipeline and an automation tool, PHP a tiny web framework, Lua game scripting, QML a notes app. The separate "in Practice" plan for Go, Python, PHP, Lua and QML is dropped; its ideas become the stage before each project.
- Comparing stays, family first: compiled (Rust, Go), JavaScript (TypeScript, QML), scripting (Python, Lua, PHP). Other tracks are one step away, with a note on why they differ.
- `learn init` also asks for a goal: learn one language, switch from one you know, or compare.
- Progress stays keyed by track and exercise, so nobody loses work.

**Engine and interfaces**
- A `learn-core` crate in `runner/core` holds the engine (course, paths, specs, progress, notes, checks, backends) and returns plain, serialisable view data. It has no terminal or HTTP code. `runner/Cargo.toml` stays the `learn` binary and becomes the workspace root, so CI and release paths don't change.
- The `learn` binary draws that data as the CLI, a ratatui TUI (opened by plain `learn` on a terminal; `--no-tui` or `NO_TUI=1` skips it), and the Nuxt web UI (three columns: path, Lesson/Task/Compare, live tests; an "All tracks" page for the old grid).
- Editors: `learn lsp`, a language server that shows failing cases as diagnostics, hints on hover and check/hint/lesson code actions, plus a generated `LESSON.md` beside each stub. A thin Zed extension starts it (Zed extensions can't add panels yet). *Changed 2026-11-11:* the extension lives in this repo (`editors/zed`) and isn't published to Zed's registry, since it's only useful to learners; release builds of `learn` embed it, and `learn init` / `learn zed` install it.
- Later: `learn mcp` for AI assistants. Not now: a Tauri desktop app.

**Releases**: 0.4.0 the engine, paths and new web UI; 0.5.0 the TUI and editor support; then one project at a time.

## Consequences

- Content can be written for what each language is good at instead of what all seven share.
- Every interface gets new features from one place; the CLI, web and LSP can't disagree.
- More files per track (`path.toml`, project stages), checked by repo tests so paths and the shared library can't drift apart.
- `course.toml` keeps every field older binaries read, so a 0.3.x binary on a newer checkout still works (it just doesn't see paths).
- Projects are most of the work: roughly 55 exercises plus reference solutions, one project per PR, each starting from an approved outline.
