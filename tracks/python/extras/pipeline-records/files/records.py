"""One order, checked and typed."""

from dataclasses import dataclass
from datetime import date
from decimal import Decimal, InvalidOperation

FIELDS = ("order_id", "date", "customer", "product", "category", "quantity", "unit_price", "country")


# TODO: make this a frozen dataclass, so orders can't change and can go in a set.
class Order:
    order_id: str
    date: date
    customer: str
    product: str
    category: str
    quantity: int
    unit_price: Decimal
    country: str

    @property
    def total(self) -> Decimal:
        """unit_price × quantity, exactly."""
        raise NotImplementedError("TODO: implement Order.total")


def parse_order(row: dict[str, str]) -> Order:
    """An Order from a row of text fields (extra fields are ignored).

    Every field in FIELDS is required; surrounding spaces are stripped. date is
    YYYY-MM-DD, quantity a whole number of at least 1, unit_price a Decimal of
    zero or more, country a two-letter code (stored upper-case), and category is
    stored lower-case. Anything wrong is a ValueError whose message names the field.
    """
    raise NotImplementedError("TODO: implement parse_order")
