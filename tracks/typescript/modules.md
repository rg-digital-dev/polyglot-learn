# Modules in TypeScript

Every `.ts` file is a module. Names are private to their file unless they are marked `export`, and another file brings them in with `import { name } from "./file.ts"`. This track runs ECMAScript modules (`"type": "module"`), and because Node runs the source as is, relative imports spell out the `.ts` extension. An import of types only is written `import type`, so Node can erase it completely.

```typescript
// money/format.ts
export interface Price {
  cents: number;
  currency: string;
}

export function formatPrice(p: Price): string {
  return `${(p.cents / 100).toFixed(2)} ${p.currency}`;
}
```

```typescript
// money/cart.ts
import { formatPrice } from "./format.ts";
import type { Price } from "./format.ts";

const total: Price = { cents: 1250, currency: "EUR" };
console.log(formatPrice(total)); // 12.50 EUR
```

<!-- from:javascript -->
## Coming from JavaScript
- `import` and `export` are standard ES module syntax; only the `.ts` extension in paths is new. Editors accept it with `allowImportingTsExtensions`.
- Import types with `import type` (or `import { type Price }`). A plain `import { Price }` makes Node look for a runtime export, which fails with "does not provide an export named 'Price'".
- `namespace` blocks with runtime code and `import x = require()` are not supported by type stripping; files are the modules.
<!-- /from -->

<!-- from:default -->
## Key points
- `export` makes a name public; everything else stays inside the file.
- Relative imports start with `./` and include the `.ts` extension.
- Rename on import if needed: `import { split as words } from "./words.ts"`.
<!-- /from -->

## Common mistake
Leaving out the extension. `import { split } from "./words"` fails with `ERR_MODULE_NOT_FOUND`, because Node does not guess file extensions for ES modules.

## In this exercise
First implement `export function split(text: string): string[]` in `slugify/words.ts`; its tests run first. Then implement `export function slugify(title: string): string` in `slugify/slugify.ts`, which gets the helper with `import { split } from "./words.ts"`.
