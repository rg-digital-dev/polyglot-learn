"""Lazy steps that orders flow through. Each takes an iterable and returns an
iterator, so steps chain and nothing runs until the end of the chain asks."""

import itertools
from collections.abc import Iterable, Iterator
from datetime import date
from decimal import Decimal
from typing import TypeVar

from records import Order

T = TypeVar("T")


def between(orders: Iterable[Order], start: date | None, end: date | None) -> Iterator[Order]:
    """Orders from start to end, both included; None means no limit on that side."""
    raise NotImplementedError("TODO: implement between")


def in_category(orders: Iterable[Order], *categories: str) -> Iterator[Order]:
    """Orders in any of the categories, ignoring case. No categories keeps everything."""
    raise NotImplementedError("TODO: implement in_category")


def batched(items: Iterable[T], n: int) -> Iterator[list[T]]:
    """Lists of n items, the last one shorter if need be. n below 1 is a ValueError."""
    raise NotImplementedError("TODO: implement batched")


def running_total(orders: Iterable[Order]) -> Iterator[tuple[Order, Decimal]]:
    """Each order with the revenue so far, itself included."""
    raise NotImplementedError("TODO: implement running_total")
