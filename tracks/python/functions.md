# Functions in Python

A function is defined with `def`, a name, parameters, and an indented body; `return` hands back a value, and a function without one returns `None`. Parameters and the return value can carry type hints (`name: str`, `-> str`), which document the function and let your editor check calls, but Python itself doesn't enforce them when it runs. Strings are immutable: methods like `strip()`, `lower()` and `replace()` return a new string. f-strings (`f"Hi {name}"`) put values into text.

```python
def label(item: str, count: int = 1) -> str:
    item = item.strip()
    if count == 1:
        return f"1 {item}"
    return f"{count} {item}s"


print(label("  apple "))      # 1 apple
print(label("pear", 3))       # 3 pears
print(label(count=2, item="fig"))  # 2 figs: arguments by name, in any order
```

<!-- from:typescript -->
## Coming from TypeScript
- Blocks are indentation, not braces, and there are no semicolons.
- Type hints look like TypeScript annotations, but nothing checks them at run time; a type checker in your editor does.
- `f"Hi {name}"` is a template literal with `f` instead of backticks.
- Default values (`count: int = 1`) and keyword arguments (`label(count=2, item="fig")`) are built in.
<!-- /from -->

<!-- from:default -->
## Key points
- `def name(param: Type) -> ReturnType:` and an indented body.
- No `return` means the function returns `None`.
- String methods return new strings; the original never changes.
<!-- /from -->

## Common mistake
A mutable default value is created once and shared by every call:

```python
def add(item, items=[]):    # the same list on every call
    items.append(item)
    return items
def add(item, items=None):  # fixed: make a new list inside
    items = [] if items is None else items
    items.append(item)
    return items
```

## In this exercise
Implement `greet(name)` in `hello/hello.py`. Trim the name; when nothing is left, greet the world instead.
