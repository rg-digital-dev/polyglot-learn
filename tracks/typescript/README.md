# TypeScript track

This track runs TypeScript directly on Node.js. There is no compiler step, no
`npm install` and no dependencies: Node strips the type annotations and runs the
remaining JavaScript, and the tests use Node's built-in test runner.

## What you need

**Node.js 22.18+ or 23.6+** (any 24.x or later is fine). That is the only
requirement. Check with `node --version`.

| Node version                | TypeScript support                                     |
| --------------------------- | ------------------------------------------------------ |
| 22.18+, 23.6+, 24 and later | Type stripping on by default                           |
| 22.6 - 22.17, 23.0 - 23.5   | Needs `--experimental-strip-types` (`learn` adds it)  |
| older than 22.6             | Not supported; upgrade Node                            |

Type stripping stopped printing an "experimental" warning in 22.18 and 24.3,
and was declared stable in 24.12 and 25.2.

## Install

- **Windows:** `winget install --id OpenJS.NodeJS.LTS -e`
- **macOS:** `brew install node`
- **Arch:** `sudo pacman -S nodejs`
- **Debian / Ubuntu / Fedora:** the distro packages are often too old (Debian 13
  ships Node 20). Use [fnm](https://github.com/Schniz/fnm):
  `curl -fsSL https://fnm.vercel.app/install | bash`, open a new shell, then
  `fnm install --lts`. If your distro's `node --version` already prints 22.18 or
  newer, that works too.

## How the tests run

`learn` runs, from `workspace/typescript`:

```sh
node --test --test-reporter=tap <slug>/<slug>.test.ts
```

You can run the same command yourself. `workspace/typescript/package.json`
sets `"type": "module"`, so files use `import`/`export`, and imports must
include the `.ts` extension (`from "./hello.ts"`).

## What type stripping does not accept

Node only erases types; it never generates code. These raise
`ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` or a parse error:

- `enum` declarations (use a union of string literals or an `as const` object)
- `namespace` blocks that contain runtime code
- constructor parameter properties (`constructor(private x: number)`)
- import aliases (`import x = require(...)`, `import A = B.C`)
- decorators

Also: Node ignores `tsconfig.json` (no `paths`, no down-levelling), and
type-only imports need the `type` keyword: `import type { Foo } from "./foo.ts"`.
Node never type-checks; your editor does that.

## Editor setup

- Any editor with the TypeScript language server (VS Code, Zed, WebStorm, Neovim) works.
- For checking that matches Node's rules, a `tsconfig.json` with `"noEmit": true`,
  `"module": "nodenext"`, `"allowImportingTsExtensions": true`,
  `"erasableSyntaxOnly": true` and `"verbatimModuleSyntax": true` (TypeScript 5.8+)
  flags anything Node would reject.

## References

- [TypeScript documentation](https://www.typescriptlang.org/docs/)
- [Node.js: Modules: TypeScript](https://nodejs.org/api/typescript.html)
- [Node.js test runner](https://nodejs.org/api/test.html)
- [Exercism TypeScript track](https://exercism.org/tracks/typescript)
