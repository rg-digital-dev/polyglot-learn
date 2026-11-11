"""Orders kept in SQLite. Store money as whole cents, so sums stay exact."""

import sqlite3
from collections.abc import Iterable, Iterator
from datetime import date
from decimal import Decimal

from records import Order

SCHEMA = """
CREATE TABLE IF NOT EXISTS orders (
    order_id    TEXT PRIMARY KEY,
    date        TEXT NOT NULL,
    customer    TEXT NOT NULL,
    product     TEXT NOT NULL,
    category    TEXT NOT NULL,
    quantity    INTEGER NOT NULL,
    price_cents INTEGER NOT NULL,
    country     TEXT NOT NULL
)
"""


class Store:
    def __init__(self, path: str = ":memory:") -> None:
        """Open (or create) the database at path and make sure the table exists."""
        raise NotImplementedError("TODO: implement Store")

    def __enter__(self) -> "Store":
        raise NotImplementedError("TODO: implement Store.__enter__")

    def __exit__(self, *exc: object) -> None:
        """Close the store at the end of a with block."""
        raise NotImplementedError("TODO: implement Store.__exit__")

    def close(self) -> None:
        raise NotImplementedError("TODO: implement Store.close")

    def save(self, orders: Iterable[Order]) -> int:
        """Store the orders in one transaction; returns how many were new.
        Orders already stored (same id) are skipped. If anything fails, nothing is stored."""
        raise NotImplementedError("TODO: implement Store.save")

    def count(self) -> int:
        raise NotImplementedError("TODO: implement Store.count")

    def orders(self, start: date | None = None, end: date | None = None) -> Iterator[Order]:
        """Stored orders between start and end (both included, None for no limit), by date then id."""
        raise NotImplementedError("TODO: implement Store.orders")

    def revenue_by_country(self) -> list[tuple[str, Decimal]]:
        """Revenue per country, summed by SQLite, largest first, then by country."""
        raise NotImplementedError("TODO: implement Store.revenue_by_country")
