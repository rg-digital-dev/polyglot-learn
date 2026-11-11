"""Where orders come from: CSV and JSON Lines files, behind one Protocol."""

import json
import os
from collections.abc import Iterable, Iterator
from typing import Protocol, runtime_checkable

from reading import RowError, read_csv
from records import Order, parse_order


# TODO: let isinstance(x, Source) check an object's shape at run time.
class Source(Protocol):
    """Anything with a name and a records() method is a source."""

    name: str

    def records(self) -> Iterator[Order | RowError]: ...


class CsvSource:
    """A CSV file; name is the file's name without its folder."""

    def __init__(self, path: str) -> None:
        raise NotImplementedError("TODO: implement CsvSource")

    def records(self) -> Iterator[Order | RowError]:
        raise NotImplementedError("TODO: implement CsvSource.records")


class JsonLinesSource:
    """One JSON object per line, blank lines skipped. Values may be numbers
    where text is expected; null counts as missing. A line that isn't a JSON
    object is a RowError with its line number."""

    def __init__(self, path: str) -> None:
        raise NotImplementedError("TODO: implement JsonLinesSource")

    def records(self) -> Iterator[Order | RowError]:
        raise NotImplementedError("TODO: implement JsonLinesSource.records")


def open_source(path: str) -> Source:
    """The source for a file, by its extension (any case): .csv, or .jsonl /
    .ndjson. Anything else is a ValueError naming the extension."""
    raise NotImplementedError("TODO: implement open_source")


def merged(sources: Iterable[Source]) -> Iterator[tuple[str, Order | RowError]]:
    """Every source's records, one source after another, each with its source's name."""
    raise NotImplementedError("TODO: implement merged")
