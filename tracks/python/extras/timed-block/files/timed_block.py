from __future__ import annotations

from collections.abc import Iterator
from contextlib import contextmanager


@contextmanager
def tag(log: list[str], name: str) -> Iterator[None]:
    """Log "<name>" on entering the block and "</name>" on leaving it, however it's left."""
    raise NotImplementedError("TODO: implement tag")


class Transaction:
    """Logs "begin" on entering, then "commit" when the block finishes normally
    or "rollback" when it raises; the exception still propagates."""

    def __init__(self, log: list[str]) -> None:
        self.log = log

    def record(self, entry: str) -> None:
        """Log `entry`."""
        raise NotImplementedError("TODO: implement Transaction.record")

    def __enter__(self) -> Transaction:
        raise NotImplementedError("TODO: implement Transaction.__enter__")

    def __exit__(self, exc_type, exc, tb) -> bool:
        raise NotImplementedError("TODO: implement Transaction.__exit__")


class IgnoreValueErrors:
    """Swallows a ValueError raised in the block, logging "ignored: <message>".
    Any other exception propagates."""

    def __init__(self, log: list[str]) -> None:
        self.log = log

    def __enter__(self) -> IgnoreValueErrors:
        raise NotImplementedError("TODO: implement IgnoreValueErrors.__enter__")

    def __exit__(self, exc_type, exc, tb) -> bool:
        raise NotImplementedError("TODO: implement IgnoreValueErrors.__exit__")
