# Blocks and Control Flow in Python

A block starts after a `:` and is made of the lines indented under it. The indentation is the syntax: there are no braces and no `end`. Use four spaces. Conditions need no parentheses: `if`, `elif`, `else`. `for x in xs:` walks any iterable, and `for i in range(10):` counts from 0 to 9. `while` repeats while a condition holds, `break` leaves a loop and `continue` skips to the next round. Loops can have an `else` block that runs when the loop ends without `break`. Python 3.10+ has `match`/`case` for matching one value: cases do not fall through, and `case _:` catches the rest.

```python
for i in range(3):
    if i == 1:
        continue
    print(i)                      # --> 0   then 2

def describe(n: int) -> str:
    match n:
        case 0:
            return "zero"
        case 1 | 2:
            return "small"
        case _:
            return "big"

print(describe(0), describe(2), describe(9))  # --> zero small big
```

<!-- from:typescript -->
## Coming from TypeScript
- `elif`, not `else if`. The `:` and the indentation replace `{ }`.
- `for x in xs:` is `for...of`. There is no C-style `for (;;)`; use `range()`.
- `match` (3.10+) does not fall through, so no `break` is needed.
- `a if cond else b` is the ternary operator.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- `continue` exists. `elif` replaces `elseif`, and a `:` replaces `then ... end`.
- `match` replaces the `if ... elseif` chain for one value.
- `range(10)` starts at 0 and stops before 10.
<!-- /from -->

<!-- from:default -->
## Key points
- A `:` opens a block and indentation is its extent.
- `if` / `elif` / `else`, `for x in xs:`, `while`, `break`, `continue`.
- `range(n)` gives 0 to n - 1.
- `match` / `case` needs Python 3.10+, and `case _:` is the catch-all.
<!-- /from -->

## Common mistake
Writing `for i in range(len(xs)):` and then `xs[i]`. Loop over the items instead: `for x in xs:`. If you need the position too, use `enumerate(xs)`.
