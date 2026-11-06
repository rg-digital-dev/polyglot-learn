# Pattern matching with match

`match value:` compares a value against a list of `case` patterns, top to bottom, and runs the first one that fits. Patterns describe *shapes*, and pull out the parts they name: a **sequence pattern** like `["go", direction]` matches a two-item list whose first item is `"go"` and binds the second; `*rest` captures the remaining items. A **class pattern** like `Point(x=0, y=y)` checks the type and fields. A **mapping pattern** like `{"type": "click", "x": x}` matches any dict that has those keys (others are allowed). `|` combines alternatives, `if` adds a guard, and `case _:` matches anything. Literal strings and numbers match by equality; a bare name always captures.

```python
from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int


def where(p):
    match p:
        case Point(x=0, y=0):
            return "origin"
        case Point(x=0, y=y) | Point(x=y, y=0):
            return f"on an axis at {y}"
        case Point(x=x, y=y) if x == y:
            return "on the diagonal"
        case _:
            return "somewhere"


def command(words):
    match words:
        case ["move", ("up" | "down") as way, *rest]:
            return f"move {way} {' '.join(rest)}".strip()
        case [single]:
            return f"just {single}"
        case _:
            return "?"


print(where(Point(0, 0)), where(Point(0, 5)), where(Point(3, 3)))   # origin on an axis at 5 on the diagonal
print(command(["move", "up", "fast"]), command(["jump"]))           # move up fast just jump
```

<!-- from:typescript -->
## Coming from TypeScript
- `match` is not `switch`: cases destructure, so one `case` both tests a shape and binds variables, like a discriminated-union narrowing and destructuring in one step.
- No fall-through and no `break`.
- A bare name in a pattern always *binds* (it doesn't compare); to compare with a constant, use a dotted name (`Color.RED`) or a literal.
<!-- /from -->

<!-- from:default -->
## Key points
- Sequence, class and mapping patterns, combined with `|`, guarded with `if`.
- `*rest` in sequences; mappings match if the listed keys are present.
- The first matching case wins; end with `case _:` when anything else is an error.
<!-- /from -->

## Common mistake
Using a plain name to mean a constant:

```python
QUIT = "quit"
match word:
    case QUIT:        # binds `QUIT` to anything: matches every word (and Python reports an error for it)
        ...
    case "quit":      # fixed: a literal
        ...
```

## In this exercise
`parse_commands.py` defines the command dataclasses. Implement `parse` with sequence patterns, `describe` with class patterns, and `summarize` with mapping patterns and guards.
