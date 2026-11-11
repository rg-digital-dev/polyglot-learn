# Stage 5 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in aggregate.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

from collections import Counter
from datetime import date
from decimal import Decimal

from aggregate import Summary, monthly_revenue, revenue_by, summarize, top_customers
from records import Order

t = H()


def order(oid, day, customer, category, qty, price, country):
    return Order(oid, date.fromisoformat(day), customer, "Item", category, qty, Decimal(price), country)


ORDERS = [
    order("1", "2026-01-30", "Ada", "books", 2, "15.00", "NL"),
    order("2", "2026-02-02", "Bob", "games", 1, "60.00", "DE"),
    order("3", "2026-02-14", "Ada", "games", 1, "20.00", "NL"),
    order("4", "2026-03-01", "Cy", "books", 3, "10.00", "FR"),
    order("5", "2026-01-02", "Di", "toys", 4, "7.50", "DE"),
]


def case_24():
    got = revenue_by(ORDERS, lambda o: o.category)
    H.check(isinstance(got, Counter), f"revenue_by should return a Counter, not {type(got).__name__}")
    H.expect(dict(got), {"books": Decimal("60.00"), "games": Decimal("80.00"), "toys": Decimal("30.00")}, "revenue per category")


t.scenario("c24_revenue_by_sums_totals_per_key", case_24)


def case_25():
    H.expect(top_customers(ORDERS, 2), [("Bob", Decimal("60.00")), ("Ada", Decimal("50.00"))], "top two")
    tie = [order("a", "2026-01-01", "Zoe", "x", 1, "5", "NL"), order("b", "2026-01-01", "Al", "x", 1, "5", "NL")]
    H.expect(top_customers(tie, 5), [("Al", Decimal("5")), ("Zoe", Decimal("5"))], "a tie goes by name")


t.scenario("c25_top_customers_sorts_by_revenue_then_name", case_25)


def case_26():
    got = monthly_revenue(ORDERS)
    H.expect(list(got), ["2026-01", "2026-02", "2026-03"], "months in order")
    H.expect(got["2026-01"], Decimal("60.00"), "January")
    H.expect(got["2026-02"], Decimal("80.00"), "February")


t.scenario("c26_monthly_revenue_groups_by_year_and_month_in_orde", case_26)


def case_27():
    s = summarize(iter(ORDERS))
    H.expect((s.start, s.end), (date(2026, 1, 2), date(2026, 3, 1)), "the period")
    H.expect((s.orders, s.revenue), (5, Decimal("170.00")), "orders and revenue")
    H.expect(s.top_customers, [("Bob", Decimal("60.00")), ("Ada", Decimal("50.00")), ("Cy", Decimal("30.00"))], "top three by default (Cy and Di tie: by name)")
    H.expect(list(s.monthly), ["2026-01", "2026-02", "2026-03"], "monthly")


t.scenario("c27_summarize_counts_orders_revenue_and_period", case_27)


def case_28():
    s = summarize([])
    H.expect(s, Summary(), "an empty Summary")
    H.expect((s.orders, s.revenue, s.start), (0, Decimal(0), None), "zeros and no period")


t.scenario("c28_summarize_of_nothing_is_empty", case_28)


def case_29():
    s = summarize(ORDERS)
    H.expect(list(s.by_category.items()), [("games", Decimal("80.00")), ("books", Decimal("60.00")), ("toys", Decimal("30.00"))], "categories")
    H.expect(list(s.by_country), ["DE", "NL", "FR"], "countries: DE 90, NL 50, FR 30")


t.scenario("c29_categories_and_countries_are_sorted_biggest_firs", case_29)
t.finish()
