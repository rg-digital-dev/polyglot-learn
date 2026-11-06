# TypeScript track

This track runs TypeScript directly on Node.js. There is no build step and no `npm install` in the workspace: Node strips the type annotations and runs the remaining JavaScript, and the tests use Node's built-in test runner. A few exercises in *TypeScript in Practice* also check your types, with TypeScript 7's `tsc`.

## What you need

- **Node.js 24 or newer.** Check with `node --version`. Node 24 strips types by default and runs `using` declarations, which *TypeScript in Practice* uses. (Type stripping was declared stable in 24.12.)
- **TypeScript 7 (`tsc`)**, installed globally, for the type-check exercises (`branded-ids`, `infer-types`). Check with `tsc --version`. TypeScript 7's compiler is a native program, so a check takes a fraction of a second. The other exercises don't need it.

## Install

- **Windows:** `winget install --id OpenJS.NodeJS.LTS -e`
- **macOS:** `brew install node`
- **Arch:** `sudo pacman -S nodejs pnpm`, then `pnpm setup` (it puts pnpm's global folder on your PATH), `source ~/.bashrc` and `pnpm add -g typescript` for `tsc`.
- **Debian / Ubuntu / Fedora:** the distro packages are often too old (Debian 13 ships Node 20). Use [fnm](https://github.com/Schniz/fnm), which needs `curl` and `unzip`: `sudo apt install curl unzip` (Fedora: `sudo dnf install curl unzip`), then `curl -fsSL https://fnm.vercel.app/install | bash`, `source ~/.bashrc` and `fnm install --lts`. If your distro's `node --version` already prints 24 or newer, that works too.
- **Then, everywhere except Arch:** `npm install -g typescript` for `tsc`.

## How the tests run

`learn` runs, from `workspace/typescript`:

```sh
node --test --test-reporter=tap <slug>/<slug>.test.ts
```

For an exercise whose `exercise.toml` lists `typecheck` files, `learn` first runs `tsc --noEmit` on them (with the same settings as `tsconfig.json`) and reports the result as case 1, "types check". The `.types.ts` file it checks is never run: each `@ts-expect-error` line in it must be a type error, so it proves your types reject what they should.

You can run the same command yourself. `workspace/typescript/package.json` sets `"type": "module"`, so files use `import`/`export`, and imports must include the `.ts` extension (`from "./hello.ts"`).

## Type checking in your editor

The tests need no packages, but your editor needs Node's type definitions to know `node:test` and friends. Without them it reports "Cannot find type definition file for 'node'". To get full type checking, run this once in `workspace/typescript`:

```sh
npm install --save-dev @types/node
```

`learn` creates `package.json` only when it's missing, so the dependency stays. Type errors then show up in your editor (or with `npx tsc`); the tests still only check behaviour.

## What type stripping does not accept

Node only erases types; it never generates code. These raise `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` or a parse error:

- `enum` declarations (use a union of string literals or an `as const` object)
- `namespace` blocks that contain runtime code
- constructor parameter properties (`constructor(private x: number)`)
- import aliases (`import x = require(...)`, `import A = B.C`)
- decorators

Also: Node ignores `tsconfig.json` (no `paths`, no down-levelling), and type-only imports need the `type` keyword: `import type { Foo } from "./foo.ts"`. Node never type-checks; your editor does that.

## Editor setup

- Any editor with the TypeScript language server (VS Code, Zed, WebStorm, Neovim) works.
- For checking that matches Node's rules, a `tsconfig.json` with `"noEmit": true`, `"module": "nodenext"`, `"allowImportingTsExtensions": true`, `"erasableSyntaxOnly": true` and `"verbatimModuleSyntax": true` (TypeScript 5.8+) flags anything Node would reject.

## References

- [TypeScript documentation](https://www.typescriptlang.org/docs/)
- [Node.js: Modules: TypeScript](https://nodejs.org/api/typescript.html)
- [Node.js test runner](https://nodejs.org/api/test.html)
- [Exercism TypeScript track](https://exercism.org/tracks/typescript)
