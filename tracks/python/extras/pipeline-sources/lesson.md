# Sources behind a Protocol

The shop's orders come from two places: a CSV export and a web shop that writes JSON Lines (one JSON object per line). The rest of the pipeline shouldn't care which. A `typing.Protocol` describes the shape every source must have: here, a `name` and a `records()` method. Any class with that shape fits, without inheriting from anything.

```python
import json
from typing import Protocol, runtime_checkable

@runtime_checkable
class Greeter(Protocol):
    name: str
    def greet(self) -> str: ...

class English:
    name = "en"
    def greet(self) -> str:
        return "hello"

class Dutch:            # no base class: it just has the right shape
    name = "nl"
    def greet(self) -> str:
        return "hallo"

def all_greetings(greeters: list[Greeter]) -> list[str]:
    return [f"{g.name}: {g.greet()}" for g in greeters]

print(all_greetings([English(), Dutch()]))       # ['en: hello', 'nl: hallo']
print(isinstance(Dutch(), Greeter))              # True, thanks to @runtime_checkable
print(isinstance(object(), Greeter))             # False

for line in ['{"n": 1}', "not json"]:
    try:
        print(json.loads(line))
    except json.JSONDecodeError as e:            # a ValueError too
        print("bad line:", e)
```

A type checker (mypy, pyright) checks a Protocol before the program runs. `@runtime_checkable` also lets `isinstance` check that the members exist while it runs.

A generator that opens a file inside `with` keeps the file open exactly as long as someone is reading it. When the last item has been read, or the generator is dropped, the `with` closes the file.

<!-- from:typescript -->
## Coming from TypeScript
- A Protocol is a TypeScript `interface`: structural, so any object with the right members fits, and no `implements` is needed.
- `@runtime_checkable` adds what TypeScript can't do, a run-time `isinstance` check. It looks only at whether the members exist, not at their types.
- JSON Lines is what you'd parse with `line.split("\n").map(JSON.parse)`, but streaming and with a line number for each failure.
<!-- /from -->

<!-- from:default -->
## Key points
- `Protocol` describes a shape; classes fit it by having the members, not by inheriting.
- Choose the implementation in one place (`open_source`) and let the rest work with the Protocol.
- `yield from` hands over every item of another iterator, such as `read_csv`.
<!-- /from -->

## Common mistake
Opening the file in `__init__` and keeping it open "for later". The file stays open as long as the object lives, often until the program ends. Open it in `records()`, inside `with`.

## In this stage
In `sources.py`, make `Source` runtime-checkable, then implement `CsvSource`, `JsonLinesSource`, `open_source` and `merged`.
