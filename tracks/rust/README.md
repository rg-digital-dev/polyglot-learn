# Rust track

Small exercises in idiomatic Rust: functions and strings, conditionals, loops, maps, errors and sorting (Foundations), then structs, interfaces, closures, iterators, modules and concurrency (Building Blocks), and on through the shared modules, Deeper Rust and Rust in Practice. The path ends with a project: `logq`, a log-analysis CLI built over eight stages, from parsing and errors to arguments, streaming input, filters, output formats, threads and zero-copy parsing, with only the standard library.

## What you need

- Rust 1.88 or newer (stable; the `learn` CLI needs it too), installed through `rustup`. This gives you `rustc` and `cargo`.
- A C linker: the MSVC Build Tools on Windows, Xcode Command Line Tools on macOS, `gcc` on Linux.

Check with `cargo --version`. `learn doctor` also reports a missing toolchain.

## Install

**Windows**

```powershell
winget install --id Microsoft.VisualStudio.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --id Rustlang.Rustup -e
rustup default stable
```

**macOS** (Homebrew's `rustup` is keg-only, so put it on your `PATH`)

```sh
xcode-select --install
brew install rustup
export PATH="$(brew --prefix rustup)/bin:$PATH"   # also add this line to ~/.zshrc
rustup default stable
```

**Arch:** `sudo pacman -S --needed base-devel rustup && rustup default stable`

**Debian 13+ / Ubuntu 24.04+:** `sudo apt install build-essential rustup && rustup default stable`

**Fedora:** `sudo dnf install gcc rustup && rustup-init -y`

On any OS you can instead use the official installer from https://www.rust-lang.org/tools/install.

## Running the tests

Each exercise is its own crate, inside a numbered folder for its stage: for example `workspace/rust/01-foundations/leap-year/`. You edit `src/lib.rs`; the tests live in `tests/spec.rs`. `learn` runs them for you with:

```sh
cd workspace/rust
cargo test -p <slug> --test spec      # e.g. cargo test -p leap-year --test spec
```

You can run the same command by hand. Add `-- --nocapture` to see `println!` output from your code.

The stubs start with `todo!(...)`, which panics, so every test fails until you replace it.

The path ends with a project: `logq`, a log-analysis CLI that grows over eight stages in one crate, `workspace/rust/NN-log-cli/` (package `log-cli`, binary `logq`). Each stage adds a module and its test file (`tests/s1_parse.rs`, …) once the stage before passes; your code carries forward, and `learn` runs every test so far with `cargo test -p log-cli --tests --no-fail-fast`. `learn` writes the project's `src/lib.rs` as modules arrive, so leave it alone. `PATH.md` in `workspace/rust/` shows where you are.

In Module 2, `slugify` spans two files (`src/words.rs` and `src/lib.rs`), and `run-all` uses plain `std::thread`: no async runtime, no dependencies.

## Editor setup

- **Any LSP editor:** install rust-analyzer with `rustup component add rust-analyzer`.
- **VS Code:** install the "rust-analyzer" extension (it brings its own server).
- **Format and lint:** `cargo fmt` and `cargo clippy` (both ship with a default rustup install).

## References

- Rust documentation home: https://doc.rust-lang.org/
- The Rust Book: https://doc.rust-lang.org/book/
- Rustlings: https://github.com/rust-lang/rustlings
- Exercism Rust track: https://exercism.org/tracks/rust
