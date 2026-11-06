# Modules in Python

Every `.py` file is a module, and its name is the file name without `.py`. `import words` runs `words.py` once and gives you its top-level names as `words.split`, `words.CONSTANT`. `from words import split` copies one name in, so you can write `split(...)`. Python looks for modules in the folder of the script being run, then in the standard library and installed packages. There's no `export`: every top-level name is importable, and a leading underscore (`_helper`) marks a name as internal by convention.

```python
# geometry.py
import math

PI = math.pi

def circle_area(r: float) -> float:
    return PI * r * r

def _round(x: float) -> float:   # internal by convention
    return round(x, 2)


# main.py (in the same folder)
import geometry
from geometry import circle_area

print(geometry.PI > 3, circle_area(1.0) == geometry.circle_area(1.0))   # True True
```

<!-- from:typescript -->
## Coming from TypeScript
- No `export` keyword: everything at the top level of a file can be imported.
- `import words` is like `import * as words from "./words"`; `from words import split` is a named import.
- A file runs once on first import; later imports reuse it.
<!-- /from -->

<!-- from:default -->
## Key points
- One file, one module; the file name is the module name.
- `import module` then `module.name`, or `from module import name`.
- `_name` means "internal", by convention only.
<!-- /from -->

## Common mistake
Naming your file after a module you also need:

```python
# random.py, in your own folder
import random       # imports this file, not the standard library
random.randint(1, 6)  # AttributeError
```

## In this exercise
`slugify/words.py` is a helper module: implement `split(text)` there, then `slugify(title)` in `slugify/slugify.py`, which already does `import words`.
