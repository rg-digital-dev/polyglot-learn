# polysyntax

*Formerly polyglot-learn.*

Learn another programming language on your own machine with small, test-driven exercises. It's free, it works offline, and you use your own editor.

Every exercise is written **once** as language-neutral data: a signature plus test cases. Each language track generates its own stub and native tests from that data. You fix a stub, the tests go green, and you move on. Because the exercises are shared, you can also do the same exercise in several languages and compare them.

**Who it's for:** working developers picking up another language. The notes explain each idea relative to a language you already know (TypeScript/JavaScript to start with), instead of from scratch.

**Tracks:** TypeScript (Node), Python 3.12+, Rust, Go, PHP, Lua 5.4 and QML (Qt 6).

**Modules:** *Foundations* (functions, conditionals, loops, maps, errors, sorting) and *Building Blocks* (structs, interfaces, closures, iterators, modules, concurrency). There are 12 exercises, each available in every track, followed by *Values and References* (`make-grid` and `add-bonus`: when two names share one list, and shallow vs. deep copies), *Text and Unicode* (`char-count` and `truncate`: bytes, UTF-16 units and code points, and cutting text without splitting a character), *Numbers and Division* (`wrap-index`, `split-evenly`, `round-percent` and `checked-multiply`: division and remainders with negative numbers, rounding halves, and overflow), *Real-world Basics* (`error-context`, `parse-config`, `int-setting` and `validate-signup`: adding context to an error without losing its cause, saying which line failed, telling a missing setting from a broken one, and reporting every problem at once) and *Files and Resources* (`read-file`, and `close-everything`: closing every resource on every way out), which is on every track except QML, since QML can't read files without C++. Then each track has its own *Deeper* module (4 to 6 exercises) for what doesn't exist everywhere: ownership, lifetimes, enums and smart pointers in Rust; embedding, channels, `select` and generics in Go; unions, narrowing and generators in TypeScript; generators, decorators, context managers, `match` and dunder methods in Python; enums, readonly objects and traits in PHP; metatables and coroutines in Lua; bindings, signals and models in QML. Rust and TypeScript continue with *in Practice* modules: `Drop` guards, error conversion, custom iterators, `Cow`, statics and channels in Rust; cancellation with `AbortSignal`, `using`, branded types and conditional types in TypeScript (the last two are type-checked with TypeScript 7's `tsc`).

## Quick start

Go to the folder where you keep your projects. One command installs the `learn` CLI and fetches the course into a `polysyntax` folder there:

```bash
curl -fsSL https://raw.githubusercontent.com/rg-digital-dev/polysyntax/main/install.sh | sh
```

On Windows, in PowerShell:

```powershell
irm https://raw.githubusercontent.com/rg-digital-dev/polysyntax/main/install.ps1 | iex
```

The installer picks the [release](https://github.com/rg-digital-dev/polysyntax/releases/latest) binary for your system (Linux, macOS and Windows, x86-64 and ARM), checks it against the published checksums, and ends with `learn doctor`, which lists the language toolchains to install. To update, run it again from the same folder (or from inside `polysyntax`); your work stays. Read [install.sh](install.sh) or [install.ps1](install.ps1) first if you like; the top of each lists its settings, such as other install folders.

Then:

```bash
cd polysyntax
learn init          # which language do you already know?
learn start rust    # or typescript, python, go, php, lua, qml
learn               # the full-screen view: edit the file, save, watch the tests
```

**By hand:** download `learn` from the latest release, put it on your `PATH`, and `git clone` this repository. The binaries aren't signed: on macOS, allow it once under System Settings > Privacy & Security; on Windows, choose "More info" > "Run anyway" if SmartScreen asks.

**From source**, with Rust 1.88 or newer: clone the repository and run `cargo install --locked --path runner` inside it.

Plain `learn` opens a full-screen view: your path on the left, the lesson and task in the middle, and the tests on the right. Press `e` to open the file in your editor (`$EDITOR`, or Zed); saving it runs the tests, and they tick off one by one. `?` lists the keys. Prefer plain output? `learn watch` re-runs the tests on every save in the terminal, and `--no-tui` (or `NO_TUI=1`) makes `learn` print the task instead. Every exercise folder also has a `LESSON.md` with the task and the lesson, to read in your editor's Markdown preview. The first Rust check compiles for a few seconds; after that it's quick.

## Web UI

Prefer a browser? Run:

```bash
learn serve --open
```

This opens `http://learn.localhost:4321`. It shows the same course with rendered lessons, a progress grid, test cases turning green or red live as they run, hints, a side-by-side compare view, and an optional in-page editor. You can keep coding in your own editor: the page re-runs the tests when you save.

![The exercise page: lesson on the left, tests passing and failing on the right](docs/web-ui.jpg)

- **It only runs while the command runs.** Ctrl+C or the Quit button stops it, and no daemon or background process is left behind.
- **It only listens on `127.0.0.1`,** because it runs code. Requests for any other host name are refused.
- **It gives the same results as `learn check`,** because both use the same code.
- **The in-page editor never overwrites a newer file.** If the file changed on disk since you opened it, the page asks before reloading or overwriting.

## Commands

| Command | What it does |
|---|---|
| `learn` | On a terminal, the full-screen view (path, lesson and task, live tests); otherwise `learn show` |
| `learn show [slug]` | The current exercise: what to do, the file to edit, what to run next (`--full` adds the lesson) |
| `learn lesson [slug]` | The concept lesson, your track's note and further reading, in a pager when it's long |
| `learn read [page] [--lang <id>]` | The next page of the Syntax Primer (or the one you name), with your track's note |
| `learn check [slug] [-v]` | Run the tests; mark the exercise done when they pass (`-v` for every detail) |
| `learn watch` | Re-run on save, advance automatically |
| `learn hint [slug]` | Reveal the next hint |
| `learn status [--all]` | Your progress on the active track (`--all`: every track side by side) |
| `learn compare <concept> [--all \| --lang a,b]` | One concept's note from each track, one after another; starts with your track's family |
| `learn docs [concept] [--open]` | Official docs, *lings and Exercism links for a concept |
| `learn start <lang>` | Switch track (your work on other tracks is kept) |
| `learn init [--from <lang>] [--goal learn\|switch\|compare]` | The language you know and what you're here for: learning one language, switching from one you know (notes lead with what's different), or comparing (`--compare rust,go`) |
| `learn doctor` | Which toolchains are installed, with install hints for your OS |
| `learn generate [--lang <id\|all>]` | Regenerate tests after a spec changes; never overwrites your stubs |
| `learn serve [--port N] [--open]` | The web UI at `http://learn.localhost:4321`, until you stop it |
| `learn lsp` | A language server for your editor (see [In your editor](#in-your-editor)) |
| `learn zed [--remove]` | Install, update or remove the Zed extension |
| `learn check-links` | Report dead links in `tracks/*/track.toml` |

`learn` works from any subfolder of the repo. Output is coloured on a terminal; `--color never` (or `NO_COLOR`) turns that off, `--ascii` uses plain ASCII symbols, and `--no-pager` prints long lessons directly.

## In your editor

`learn lsp` is a small language server that runs next to your language's own one. It only answers for files inside a learn workspace:

- **Save** and it runs the tests. Failing cases show as warnings on the function's signature, for example "case 3, empty name" with the call, the expected value and what came back. A compile error shows as one error.
- **Hover** the signature for the task, what the tests check and the hints you've revealed.
- **Code actions:** "Check this exercise", "Show next hint" (it counts, like `learn hint`), "Open LESSON.md" and "Open next exercise".

Any editor that can run a language server over stdio can use it: point it at `learn lsp` for the languages you're learning. In **Zed**, `learn` does it for you: `learn init` offers the polysyntax extension when Zed is installed, and `learn zed` installs or updates it. Restart Zed, open your checkout, and save an exercise; hover a function's name for the task. It's not in Zed's extension registry, since it's only useful here; see [editors/zed](editors/zed/README.md) for settings and troubleshooting. `learn` also writes a `.zed/tasks.json` with **learn check** and **learn watch** (for `task: spawn`) when it sets up a track, unless the folder already has one.

Editors list folders by name, so each track's workspace folder is laid out in the order of your path: one numbered folder per stage (`01-foundations/`, `02-building-blocks/`, …), with that stage's exercises inside. `PATH.md` beside them lists the whole path with what you've done, marks where you are, and links every exercise and lesson; open it in your editor's Markdown preview. Workspaces from older versions move into this layout the next time `learn` sets up the track, your code included.

## Keeping your own solutions

Your solutions and progress are **never** part of this repo: `workspace/` and `.learn/` are gitignored. There are two ways to keep them:

- **Separate folder (recommended).** Keep your solutions in their own folder, and their own git repo if you like:
  ```bash
  learn --workspace ~/code/my-solutions start go
  ```
  You can also set `LEARN_WORKSPACE=~/code/my-solutions`. Progress is saved in that folder too, and `learn` works when run from inside it.
- **Fork.** Fork this repo and work in `workspace/` as usual. Pull new exercises from upstream with `git pull upstream main`.

## How it works

```
course.toml            shared modules -> ordered exercise slugs
tracks/<lang>/path.toml  the track's stages: shared modules it uses, its own stages
families.toml          track families and why they differ
specs/<slug>.json      language-neutral exercise: signature + test cases
concepts/<concept>.md  the lesson, written once
tracks/<lang>/         README, track.toml (links, install hints), one note per concept
tracks/<lang>/extras/  track-only exercises, written natively (lesson, stub, tests)
runner/                the `learn` CLI (Rust): one backend per language
workspace/<lang>/      generated: PATH.md, then one numbered folder per stage of your path
                       (01-foundations/hello/…) with your stub, the tests and a LESSON.md (gitignored)
```

A backend maps the small spec type vocabulary (`int`, `bool`, `string`, `list<T>`, `map<string,T>`, plus records, classes, interfaces and function types) to native types, renders typed literals, writes the stub and the tests, and reads per-case results back from the test output. Errors follow each language's own convention: `Result` in Rust, `(T, error)` in Go, exceptions in TypeScript, PHP and QML, and `nil, err` in Lua.

Track-only exercises skip the generation step: their stub and tests are written by hand in the language itself, and `learn` copies them into the workspace and runs them with the same test command as the generated ones.

The design borrows from [Exercism's problem-specifications](https://github.com/exercism/problem-specifications) (exercises as data, generated per-language tests) and from [Rustlings](https://github.com/rust-lang/rustlings) (fix one small file at a time, with a watcher). The exercises and text here are original.

## Adding an exercise

Add `specs/<slug>.json` and list the slug in a `course.toml` module. Every track whose path uses that module now has the exercise; run `learn generate --lang all`. Track notes for a new concept are optional. For a feature only one language has, write a track-only exercise in `tracks/<lang>/extras/<slug>/` instead. See [CONTRIBUTING.md](CONTRIBUTING.md).

## Adding a language

Write one backend module in `runner/core/src/backends/` and a `tracks/<lang>/` folder. [CONTRIBUTING.md](CONTRIBUTING.md) has a checklist.

## Licence

- **Code** (`runner/`, specs, generated harnesses): [MIT](LICENSE).
- **Lesson text** (`concepts/`, `tracks/**/*.md`): [CC BY-SA 4.0](LICENSE-CONTENT.md). You may reuse and adapt it, including commercially, as long as you credit the source and share your changes under the same licence.

The split keeps the code easy to embed anywhere, while improvements to the lessons stay open.

The web UI bundles the fonts Archivo, Geist and Martian Mono under the [SIL Open Font License 1.1](runner/web/app/assets/brand/fonts/) (licence files included). Its visual style uses RG Digital's design tokens.

---

Built by [RG Digital](https://rg-digital.dev).
