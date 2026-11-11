"""Adding orders up."""

from collections import Counter, defaultdict
from collections.abc import Callable, Iterable
from dataclasses import dataclass, field
from datetime import date
from decimal import Decimal

from records import Order


def revenue_by(orders: Iterable[Order], key: Callable[[Order], str]) -> Counter[str]:
    """Total revenue per key(order), as a Counter."""
    raise NotImplementedError("TODO: implement revenue_by")


def top_customers(orders: Iterable[Order], n: int) -> list[tuple[str, Decimal]]:
    """The n customers who spent most, with what they spent: largest first, ties by name."""
    raise NotImplementedError("TODO: implement top_customers")


def monthly_revenue(orders: Iterable[Order]) -> dict[str, Decimal]:
    """Revenue per month ("2026-01"), months in order."""
    raise NotImplementedError("TODO: implement monthly_revenue")


@dataclass
class Summary:
    """Everything the report needs. The dicts are ordered: categories and
    countries by revenue (largest first, ties by name), months by date."""

    start: date | None = None
    end: date | None = None
    orders: int = 0
    revenue: Decimal = Decimal(0)
    by_category: dict[str, Decimal] = field(default_factory=dict)
    by_country: dict[str, Decimal] = field(default_factory=dict)
    top_customers: list[tuple[str, Decimal]] = field(default_factory=list)
    monthly: dict[str, Decimal] = field(default_factory=dict)


def summarize(orders: Iterable[Order], top: int = 3) -> Summary:
    """A Summary of any iterable of orders (it may be a generator, read once).
    No orders gives Summary()."""
    raise NotImplementedError("TODO: implement summarize")
