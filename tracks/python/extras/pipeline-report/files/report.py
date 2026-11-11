"""A Markdown report from a Summary."""

from collections.abc import Sequence
from decimal import ROUND_HALF_UP, Decimal

from aggregate import Summary


def money(amount: Decimal) -> str:
    """Two decimals, rounded half up, thousands grouped: 1,234.50."""
    raise NotImplementedError("TODO: implement money")


def table(headers: Sequence[str], rows: Sequence[Sequence[str]], align: str) -> str:
    """A Markdown table with no trailing newline. `align` has an "l" or "r" per
    column. Every column is as wide as its widest cell (at least 3); cells are
    padded on the right ("l") or left ("r"). The rule under the headers is
    ":---" for a left column and "---:" for a right one, as wide as the column.

        | Name  |  Revenue |
        | :---- | -------: |
        | games | 1,000.00 |
    """
    raise NotImplementedError("TODO: implement table")


def render(summary: Summary, title: str = "Sales report") -> str:
    """The whole report, ending with one newline. Parts are separated by a blank line:

        # <title>
        Period: <start> to <end> · <n> orders · revenue <money>
        ## By category     (table: Category | Revenue, "lr")
        ## By country      (Country | Revenue)
        ## Top customers   (Customer | Revenue)
        ## By month        (Month | Revenue)

    With no orders, it's just the title and "No orders.". One order says "1 order".
    """
    raise NotImplementedError("TODO: implement render")
