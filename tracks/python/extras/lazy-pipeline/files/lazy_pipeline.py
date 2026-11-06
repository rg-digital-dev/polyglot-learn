from __future__ import annotations

from collections.abc import Iterable, Iterator
from typing import Any


def countdown(n: int) -> Iterator[int]:
    """Yield n, n - 1, ..., 1 (nothing for n < 1)."""
    raise NotImplementedError("TODO: implement countdown")


def naturals() -> Iterator[int]:
    """Yield 1, 2, 3, ... forever."""
    raise NotImplementedError("TODO: implement naturals")


def take(n: int, items: Iterable[Any]) -> list[Any]:
    """The first n items as a list. Pull no more than n items from `items`."""
    raise NotImplementedError("TODO: implement take")


def chunks(items: Iterable[Any], size: int) -> Iterator[list[Any]]:
    """Yield lists of `size` items; the last one may be shorter (but never empty)."""
    raise NotImplementedError("TODO: implement chunks")


def flatten(nested: list[Any]) -> Iterator[Any]:
    """Yield every non-list value in `nested`, depth first, in order."""
    raise NotImplementedError("TODO: implement flatten")
