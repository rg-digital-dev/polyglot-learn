# Reading CSV lazily with generators

A function with `yield` in it is a generator. Calling it runs nothing; it returns an iterator, and the body runs only as far as the next `yield` each time someone asks for an item. That makes it the natural shape for reading data: one row in, one record out, and a million-row file never has to fit in memory.

```python
import csv
import io

def numbers(lines):
    """Each row's "n" as an int, or an error string for a bad one."""
    reader = csv.DictReader(lines)
    for row in reader:
        try:
            yield int(row["n"])
        except ValueError:
            yield f"line {reader.line_num}: {row['n']!r} isn't a number"

text = io.StringIO('name,n\nada,1\n"lovelace, ada",x\nbob,3\n')
gen = numbers(text)
print(gen)          # <generator object numbers at 0x...>: nothing has run yet
print(next(gen))    # 1
print(list(gen))    # ["line 3: 'x' isn't a number", 3]
```

`csv.DictReader` reads the header line and then gives each row as a dict. It handles quoting, so `"Lovelace, Ada"` stays one field. Splitting on commas yourself gets this wrong. Any iterable of lines will do: an open file, a list, or `io.StringIO` in a test.

Yielding the error instead of raising it lets one bad row be reported while the rest are still imported. The caller then decides what to do with the errors.

<!-- from:typescript -->
## Coming from TypeScript
- Python generators are JavaScript's `function*`: `yield` and laziness work the same way, and `for … in` drives them as `for … of` does.
- `Order | RowError` in a type hint is the union type you know. `isinstance(item, RowError)` is the type guard that narrows it.
- There's no built-in CSV parser in Node. Python ships one, and it follows RFC 4180 quoting.
<!-- /from -->

<!-- from:default -->
## Key points
- A generator function (`yield`) is lazy: it does work only when asked for the next item.
- Use `csv.DictReader` for CSV; it handles headers and quoting.
- Yield errors as values when one bad record shouldn't stop the rest.
<!-- /from -->

## Common mistake
Building a list and returning it (`return [parse(r) for r in reader]`). It works for small files, but the whole file is read before the first order comes out, and an endless or very large input never comes out at all. Use `yield`.

## In this stage
In `reading.py`, implement `read_csv`, `split_valid` and `dedupe`. `read_csv` and `dedupe` must be generators.
