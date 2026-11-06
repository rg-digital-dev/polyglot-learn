from __future__ import annotations

import functools
from collections.abc import Callable
from typing import Any


def shout(fn: Callable[..., str]) -> Callable[..., str]:
    """A decorator: the wrapped function returns fn's result in upper case."""
    raise NotImplementedError("TODO: implement shout")


def count_calls(fn: Callable[..., Any]) -> Callable[..., Any]:
    """A decorator: the wrapper has a `calls` attribute counting how often it was called."""
    raise NotImplementedError("TODO: implement count_calls")


def retry(times: int) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """A decorator factory: @retry(3) calls the function up to 3 times while it raises.

    Return the first successful result; if every attempt raises, raise the last exception.
    """
    raise NotImplementedError("TODO: implement retry")
