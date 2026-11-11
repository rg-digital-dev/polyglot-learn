"""The command line: `python cli.py import FILE... --db DB` and `python cli.py report --db DB`."""

import argparse
import sys
from datetime import date

from aggregate import summarize
from reading import RowError
from report import render
from sources import merged, open_source
from store import Store
from transform import between, in_category


def main(argv: list[str] | None = None) -> int:
    """Run the command line and return the exit code (argv None means sys.argv[1:]).

    import FILE... --db DB
        Read every file (open_source), print each bad row on stderr as
        "<file>:<line>: <message>", save the good orders, and print
        "imported <new> new orders from <files> files (<skipped> rows skipped)".
        Exit 1 if any row was skipped, else 0.
    report --db DB [--from YYYY-MM-DD] [--to YYYY-MM-DD] [--category C]... [--out FILE]
        The Markdown report of the stored orders in that period and those
        categories, on stdout, or written to FILE followed by "wrote <FILE>".

    A usage error (argparse) returns 2 instead of ending the program.
    """
    raise NotImplementedError("TODO: implement main")


if __name__ == "__main__":
    sys.exit(main())
