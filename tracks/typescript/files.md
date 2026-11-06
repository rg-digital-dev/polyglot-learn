# Reading files in TypeScript

Node's file functions live in `node:fs`. `readFileSync(path, "utf8")` reads a whole file and returns a string; without the encoding you get a `Buffer`. When the file can't be read it throws an error whose `code` says why (`"ENOENT"` for a missing file) and whose message includes the path. The asynchronous version, `await readFile(path, "utf8")` from `node:fs/promises`, is what servers use so they don't block while waiting for the disk. The tests here call your function directly, so the synchronous one is fine.

```typescript
import { readFileSync } from "node:fs";

function firstLine(path: string): string {
  const text = readFileSync(path, "utf8");
  return text.split("\n")[0].trim();
}

try {
  firstLine("does-not-exist.txt");
} catch (e) {
  console.log((e as Error).message); // ENOENT: no such file or directory, open '<full path>/does-not-exist.txt'
}
```

<!-- from:javascript -->
## Coming from JavaScript
- The same `node:fs` API. Import with the `node:` prefix and named imports.
- Pass `"utf8"` to get a `string`; TypeScript types the result by that argument.
- The thrown value is typed `unknown` in `catch`: narrow it before reading `code` or `message`.
<!-- /from -->

<!-- from:default -->
## Key points
- `readFileSync(path, "utf8")` for a whole text file; `readFile` from `node:fs/promises` for the async version.
- Errors throw, with the path in the message and the reason in `code`.
- `split("\n")` plus `trim()` handles `\r\n` endings.
<!-- /from -->

## Common mistake
Forgetting the encoding:

```typescript
const text = readFileSync(path); // a Buffer, not a string: text.split is not a function
```

## In this exercise
`readLines(path)` reads the file and returns the trimmed lines that aren't blank or `#` comments. The tests pass the full path of files `learn` puts next to them.
