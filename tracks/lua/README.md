# Lua track

This track targets **Lua 5.4**, the reference interpreter from lua.org. Each exercise is a plain module file; the tests use a tiny harness that `learn` generates, so there is nothing else to install (no LuaRocks, no busted).

## What you need

A `lua` 5.4 interpreter. `learn` looks for `lua5.4`, `lua54`, `lua` and `luajit` and warns if the version is not 5.4. Check with `lua -v` (or `lua5.4 -v`). Lua 5.3 and 5.5 run nearly all of it; LuaJIT and Lua 5.1 lack `//`, `math.type` and the integer subtype.

## Install

- **Windows:** `winget install --id DEVCOM.Lua -e` (Lua 5.4), then open a new terminal.
- **macOS:** Homebrew's `lua` formula is now 5.5, so install 5.4 with `brew install lua@5.4`. It's keg-only, so put it on your `PATH`: `export PATH="$(brew --prefix lua@5.4)/bin:$PATH"` (also add that line to `~/.zshrc`).
- **Arch:** `sudo pacman -S lua54` (the `lua` package is 5.5). The binary is `lua5.4`.
- **Debian / Ubuntu:** `sudo apt install lua5.4`. The binary is `lua5.4`.
- **Fedora:** `sudo dnf install lua` (5.4 on Fedora 43 and 44).

## How the tests run

`learn` runs, from `workspace/lua`:

```sh
lua <slug>/<module>_test.lua      # e.g. lua leap-year/leap_year_test.lua
```

You can run the same command yourself. Your code goes in `<slug>/<module>.lua`, where the module name is the slug with underscores. It returns a table `M` of functions. The shared `harness.lua` calls each one under `pcall` and compares results, tables included, key by key. For fallible functions, return `nil, "message"`; raising with `error()` fails the test.

## Lua 5.4 details that matter here

- Indexes start at 1, and `#t` is only meaningful for sequences without holes.
- Numbers are integers or floats: `7 // 2` is `3`, `7 / 2` is `3.5`, `math.type(x)` tells which.
- `pairs` iterates in an unspecified order.
- Variables are global unless declared `local`.

## Editor setup

- [lua-language-server](https://luals.github.io/) (LuaLS) gives completion and diagnostics in VS Code, Neovim, Zed and others. The generated stubs carry `--- @param` / `--- @return` annotations that LuaLS understands. If it assumes another Lua version, set `runtime.version` to `"Lua 5.4"` (for example in a `.luarc.json` at the workspace root).

## Where Lua is used

- **Hyprland** switched its configuration to Lua in v0.55: the config file is now `~/.config/hypr/hyprland.lua`. It embeds Lua 5.5; what you learn here carries over, apart from small changes such as read-only `for` loop variables.
- **FiveM** (GTA V multiplayer) scripts run on CfxLua, a modified Lua 5.4 that adds vector types and compile-time hashes.
- **Neovim** plugins and configs, and **Redis** scripts. Both use the Lua 5.1 language (Neovim through LuaJIT), so `//` and `math.type` are not available there.
- Game modding and embedded scripting in many engines and applications.

## References

- [Lua 5.4 Reference Manual](https://www.lua.org/manual/5.4/)
- [Programming in Lua, 1st edition](https://www.lua.org/pil/contents.html) (written for Lua 5.0; free online)
- [Lua 5.5 changes](https://www.lua.org/manual/5.5/readme.html#changes)
- [Hyprland example config (hyprland.lua)](https://github.com/hyprwm/Hyprland/blob/main/example/hyprland.lua)
- [FiveM: Scripting in Lua](https://docs.fivem.net/docs/scripting-manual/runtimes/lua/)
- [Neovim Lua guide](https://neovim.io/doc/user/lua-guide/)
- [Exercism Lua track](https://exercism.org/tracks/lua)
