# Variables and Constants in Python

You create a variable by assigning to it: `x = 1`. There is no keyword and no declaration. Assigning again points the name at a new value, even one of a different type. Python has no real constants. By convention an `UPPER_CASE` name means "do not change this", and `Final` from `typing` records that for a type checker, but the interpreter enforces neither. Scope is the function: a name created inside an `if` or a `for` is still there after it, and a name assigned inside a function is local to that function.

```python
from typing import Final

count = 1
count = "now text"              # allowed: the name just points elsewhere
print(count)                    # --> now text

MAX: Final = 3
MAX = 4                         # runs: Final is only a hint for tools
print(MAX)                      # --> 4

if True:
    inside = "still here"
print(inside)                   # --> still here
```

<!-- from:typescript -->
## Coming from TypeScript
- No `let` or `const`. `x = 1` creates the variable the first time and reassigns it afterwards.
- `Final` looks like `const` but nothing stops `MAX = 4` when the program runs.
- No block scope: a variable made inside `if` or `for` survives it, like `var`.
- A typo in a name on the left of `=` silently creates a new variable.
<!-- /from -->

<!-- from:rust -->
## Coming from Rust
- Every name can be reassigned, and no `mut` is needed. Lists and dicts can be changed through any name.
- `ys = xs` copies nothing: both names share one list (see the common mistake below).
<!-- /from -->

<!-- from:default -->
## Key points
- Assign to create: `x = 1`. There is no declaration.
- Names can be rebound to any type.
- No constants are enforced: `UPPER_CASE` is a convention and `Final` is a hint.
- Scope is the function, not the block.
<!-- /from -->

## Common mistake
Assuming `b = a` copies a list. Both names point at the same list, so `b.append(1)` changes what `a` sees. Use `a.copy()` or `list(a)` for an independent list.
