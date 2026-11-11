# Go track

Small exercises in idiomatic Go: functions and strings, conditionals, loops, maps, errors and sorting (Foundations), then structs, interfaces, closures, iterators, modules and concurrency (Building Blocks), and on through the shared modules and Deeper Go. The path ends with a project: a URL shortener HTTP service built over eight stages, from a store to handlers, middleware, concurrency, timeouts, expiry and graceful shutdown, with only the standard library.

## What you need

- Go 1.22 or newer. The notes use the `slices` and `cmp` packages, the built-in `min`, and `range` over an integer.

Check with `go version`. `learn doctor` also reports a missing toolchain.

## Install

**Windows:** `winget install --id GoLang.Go -e` (open a new terminal afterwards so `go` is on your `PATH`)

**macOS:** `brew install go`

**Arch:** `sudo pacman -S go`

**Debian 13+ / Ubuntu 24.04+:** `sudo apt install golang-go`. Older releases ship a Go that is too old; use the tarball from https://go.dev/doc/install instead.

**Fedora:** `sudo dnf install golang`

On any OS you can also use the official installers from https://go.dev/doc/install.

## Running the tests

All exercises share one module, `workspace/go/go.mod`. Each exercise is a package in its own folder, inside a numbered folder for its stage: for example `workspace/go/01-foundations/leap-year/` holds package `leapyear`, with your code in `leapyear.go` and the tests in `leapyear_test.go`. `learn` runs them for you with:

```sh
cd workspace/go
go test ./<stage>/<slug>/          # e.g. go test ./01-foundations/leap-year/
```

You can run the same command by hand. Add `-v` to see each test case by name, or `-run 'TestName'` to run just one.

The stubs start with `panic(...)`, so every test fails until you replace it.

In Module 2, `slugify` has a subpackage: `slugify/words/words.go` (package `words`), which `slugify/slugify.go` imports as `"learn/slugify/words"`. An exercise with subpackages is a small module of its own (`learn/slugify`), tied to the others by `workspace/go/go.work`, so its import path doesn't depend on the stage folder.

The URL shortener project lives in one folder, `workspace/go/NN-url-shortener/`, package `shortener`. Each stage adds its files when the stage before passes, your code carries forward, and `learn` runs every test so far with `go test ./NN-url-shortener/...`. `PATH.md` in `workspace/go/` shows where you are.

## Editor setup

- **Any LSP editor:** install gopls with `go install golang.org/x/tools/gopls@latest`.
- **VS Code:** install the "Go" extension by the Go team; it offers to install gopls and other tools.
- **Format and vet:** `gofmt -w .` and `go vet ./...` (both ship with Go).

## References

- Go documentation home: https://go.dev/doc/
- A Tour of Go: https://go.dev/tour/
- golings: https://github.com/mauricioabreu/golings
- Exercism Go track: https://exercism.org/tracks/go
