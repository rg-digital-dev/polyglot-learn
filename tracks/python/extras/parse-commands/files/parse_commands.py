from __future__ import annotations

from dataclasses import dataclass
from typing import Any

DIRECTIONS = ("north", "south", "east", "west")


@dataclass
class Go:
    direction: str


@dataclass
class Take:
    item: str


@dataclass
class Drop:
    item: str


@dataclass
class Quit:
    pass


Command = Go | Take | Drop | Quit


def parse(line: str) -> Command:
    """Parse a command with `match` on its words:

    "go <direction>" (one of DIRECTIONS) -> Go; "take <one or more words>" -> Take
    (the words joined by single spaces); "drop <item>" -> Drop; "quit" or "exit" -> Quit.
    Anything else raises ValueError("unknown command: <line>").
    """
    raise NotImplementedError("TODO: implement parse")


def describe(command: Command) -> str:
    """With class patterns: "you go <direction>", "you take <item>", "you drop <item>", "bye"."""
    raise NotImplementedError("TODO: implement describe")


def summarize(event: dict[str, Any]) -> str:
    """With mapping patterns:

    {"type": "login", "user": u}                  -> "<u> logged in"
    {"type": "purchase", "user": u, "items": [..]} -> "<u> bought <n> item(s)" ("1 item", "3 items")
    {"type": "error", "code": int >= 500}         -> "server error <code>"
    {"type": "error", "code": any other int}      -> "client error <code>"
    any other dict with a "type"                  -> "unknown event <type>"
    anything else raises ValueError("bad event").
    """
    raise NotImplementedError("TODO: implement summarize")
