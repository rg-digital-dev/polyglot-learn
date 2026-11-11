# Stage 7 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in report.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

from datetime import date
from decimal import Decimal

from aggregate import Summary
from report import money, render, table

t = H()

SUMMARY = Summary(
    start=date(2026, 1, 2),
    end=date(2026, 3, 1),
    orders=5,
    revenue=Decimal("1234.5"),
    by_category={"games": Decimal("1000"), "books": Decimal("234.5")},
    by_country={"DE": Decimal("1234.5")},
    top_customers=[("Bob", Decimal("1000")), ("Ada", Decimal("234.5"))],
    monthly={"2026-01": Decimal("234.5"), "2026-03": Decimal("1000")},
)


def case_36():
    H.expect(money(Decimal("1234.5")), "1,234.50", "thousands and two decimals")
    H.expect(money(Decimal("0")), "0.00", "zero")
    H.expect(money(Decimal("1234567.891")), "1,234,567.89", "rounded")
    H.expect(money(Decimal("2.345")), "2.35", "half rounds up")
    H.expect(money(Decimal("-3.456")), "-3.46", "negative")


t.scenario("c36_money_has_two_decimals_and_grouped_thousands", case_36)

t.equals(
    "c37_table_pads_and_aligns_columns",
    lambda: table(["Name", "Revenue"], [["games", "1,000.00"], ["books", "234.50"]], "lr"),
    "| Name  |  Revenue |\n"
    "| :---- | -------: |\n"
    "| games | 1,000.00 |\n"
    "| books |   234.50 |",
)


def case_38():
    text = render(SUMMARY)
    H.check(text.startswith("# Sales report\n\n"), f"the report starts with its title: {text[:40]!r}")
    H.check("Period: 2026-01-02 to 2026-03-01 · 5 orders · revenue 1,234.50\n" in text, "the period line")
    H.check(render(SUMMARY, "Q1").startswith("# Q1\n"), "the title can be changed")
    H.check(text.endswith("\n") and not text.endswith("\n\n"), "it ends with exactly one newline")


t.scenario("c38_render_shows_the_period_count_and_revenue", case_38)


def case_39():
    text = render(SUMMARY)
    headings = [l for l in text.splitlines() if l.startswith("## ")]
    H.expect(headings, ["## By category", "## By country", "## Top customers", "## By month"], "sections in order")
    H.check(table(["Category", "Revenue"], [["games", "1,000.00"], ["books", "234.50"]], "lr") in text, "the category table")
    H.check(table(["Month", "Revenue"], [["2026-01", "234.50"], ["2026-03", "1,000.00"]], "lr") in text, "the month table")
    H.check("\n\n## By country\n\n| Country |" in text, "a blank line around each heading")


t.scenario("c39_render_has_a_table_per_section", case_39)

t.equals("c40_an_empty_summary_says_there_are_no_orders", lambda: render(Summary()), "# Sales report\n\nNo orders.\n")
t.finish()
