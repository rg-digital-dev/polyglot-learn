"""Reading orders from CSV text, a row at a time."""

import csv
from collections.abc import Iterable, Iterator
from dataclasses import dataclass

from records import Order, parse_order


@dataclass(frozen=True)
class RowError:
    """A row that isn't an order: its line number and what's wrong."""

    line: int
    message: str


def read_csv(lines: Iterable[str]) -> Iterator[Order | RowError]:
    """Every row of CSV text (with a header line) as an Order or a RowError, lazily.

    A generator: rows are read only as they're asked for. A row parse_order
    rejects becomes RowError(line number in the file, the error's message).
    """
    raise NotImplementedError("TODO: implement read_csv")


def split_valid(items: Iterable[Order | RowError]) -> tuple[list[Order], list[RowError]]:
    """The orders and the errors, each in their original order."""
    raise NotImplementedError("TODO: implement split_valid")


def dedupe(orders: Iterable[Order]) -> Iterator[Order]:
    """Each order id once: the first one seen wins. A generator."""
    raise NotImplementedError("TODO: implement dedupe")
