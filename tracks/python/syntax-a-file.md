# A File at a Glance in Python

A Python exercise file is plain top-level code. Comments start with `#`. A statement ends at the end of the line, so there are no semicolons. There is no export marker: every top-level `def` can be imported by the test file, so `def greet` is visible as it stands. The stub starts with `from __future__ import annotations`, which stops type hints from being evaluated when the file loads. The body of the function is indented under the `def` line, and the stub fills it with `raise NotImplementedError("TODO: implement greet")`, which fails on purpose. A triple-quoted string right under the `def` line is a docstring: it describes the function.

```python
from __future__ import annotations


def greet(name: str) -> str:
    """Return a greeting for name."""
    # Replace this body. print() is the quick way to look at a value.
    print(repr(name))                  # --> 'Ada'
    return "Hello, " + name + "!"


print(greet("Ada"))                    # --> Hello, Ada!
```

<!-- from:typescript -->
## Coming from TypeScript
- No `export`: any top-level function can be imported.
- No braces and no semicolons. The indentation of the body is the block.
- Comments use `#`, not `//`. A docstring is a string, not a comment.
- `print(x)` replaces `console.log(x)`.
<!-- /from -->

<!-- from:javascript -->
## Coming from JavaScript
- No `export` and no `module.exports`: top-level functions are importable as they are.
- Blocks are indentation, and `;` is not needed.
- `print(x)` replaces `console.log(x)`.
<!-- /from -->

<!-- from:default -->
## Key points
- `#` starts a comment; a statement ends at the end of the line.
- The body of a `def` is the indented block under it.
- `print(x)` shows a value; `print(repr(x))` shows it with its quotes, which helps with whitespace.
- Edit the body, never the `def` line: the tests call the function exactly as declared.
<!-- /from -->

## Common mistake
Mixing tabs and spaces, or indenting a line by a different amount than its neighbours. Python raises `IndentationError` before any code runs. Use four spaces everywhere.
