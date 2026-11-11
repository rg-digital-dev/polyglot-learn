# polysyntax for Zed

This Zed extension starts `learn lsp` for Go, Rust, Python, TypeScript, PHP, Lua and QML files in a polysyntax folder:

- **Save** an exercise and the failing test cases show up as warnings on the function they test.
- **Hover** the function's name on its signature line to see the task, what the tests check and the hints you've revealed.
- **Code actions** (`ctrl-.` / `cmd-.`) on that line: run the tests, show the next hint, open the lesson, open the next exercise.

Zed's file tree lists folders by name, and `learn` numbers each stage's folder in path order, so the tree reads top to bottom. `PATH.md` in the track folder shows the whole path with your progress; open it with `markdown: open preview`.

All the work happens in `learn`; the extension only finds it and starts it. The server only starts where there's a `course.toml` or `.learn/config.toml` at the project root, so your other projects don't get it.

## Install

It isn't in Zed's extension registry: it's for polysyntax learners, so `learn` installs it. Release builds of `learn` carry it.

- `learn init` offers it when Zed is installed, and keeps it up to date afterwards.
- `learn zed` installs or updates it; `learn zed --remove` removes it.

Restart Zed afterwards, then open your polysyntax checkout (or your own workspace folder, if you keep one outside it with `learn --workspace <folder>`).

If nothing shows up, `zed: open log` says why. The usual causes:

- **`unrecognized subcommand 'lsp'`**: the `learn` on your PATH is older than 0.5.0. Update polysyntax.
- **`server shut down`** with an `lsp.polysyntax.binary.path` setting: Zed starts that binary itself, with only the setting's `arguments`. Add `"arguments": ["lsp"]`.

## Settings

To use a `learn` that isn't on Zed's PATH, such as one built from a checkout (`runner/target/release/learn`), set it in Zed's settings (`zed: open settings`). Keep the `arguments`:

```json
{
  "lsp": {
    "polysyntax": {
      "binary": { "path": "/path/to/learn", "arguments": ["lsp"] }
    }
  }
}
```

## Develop

```sh
cargo test
cargo clippy --all-targets
cargo build --release --target wasm32-wasip2
```

The WASM build needs Rust from [rustup](https://rustup.rs) with `rustup target add wasm32-wasip2`; a Rust from another installer can't add the target. Building the runner afterwards (`cargo build` in `runner/`) embeds that build, so `learn zed` installs it. Release builds do the same in the release workflow.

To work on it inside Zed instead, remove the installed copy (`learn zed --remove`), then run `zed: install dev extension` and pick this folder. Zed compiles it with rustup; another Rust install on your PATH (such as the Windows MSI) makes that fail with `can't find crate for 'core'`. `learn zed` leaves a dev extension alone.
