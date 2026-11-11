# Stage 6 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in store.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import sqlite3
import tempfile
from datetime import date
from decimal import Decimal

from records import Order
from store import Store

t = H()


def order(oid, day, price="9.99", qty=1, country="NL"):
    return Order(oid, date.fromisoformat(day), "Ada", "Item", "books", qty, Decimal(price), country)


ORDERS = [order("B", "2026-02-01"), order("A", "2026-01-15", "0.10", 3), order("C", "2026-02-01", "1234.56", 2, "DE")]


def case_30():
    s = Store()
    H.expect(s.save(ORDERS), 3, "save returns how many were new")
    H.expect(s.count(), 3, "count")


t.scenario("c30_save_stores_orders_and_counts_them", case_30)


def case_31():
    s = Store()
    s.save(ORDERS)
    back = list(s.orders())
    H.expect([o.order_id for o in back], ["A", "B", "C"], "by date, then id")
    H.expect(back[2], ORDERS[2], "an order comes back equal: Decimal and date included")
    H.expect([o.order_id for o in s.orders(start=date(2026, 2, 1))], ["B", "C"], "from a date")
    H.expect([o.order_id for o in s.orders(end=date(2026, 1, 31))], ["A"], "up to a date")


t.scenario("c31_orders_come_back_equal_by_date", case_31)


def case_32():
    s = Store()
    s.save(ORDERS[:2])
    H.expect(s.save(ORDERS), 1, "only C is new")
    H.expect(s.count(), 3, "no duplicates")


t.scenario("c32_saving_again_skips_known_orders", case_32)


def case_33():
    s = Store()
    try:
        s.save([order("X", "2026-01-01"), None])
    except Exception:
        pass
    else:
        H.check(False, "saving None should fail")
    H.expect(s.count(), 0, "a failed save stores nothing, not even X")


t.scenario("c33_a_failed_save_stores_nothing", case_33)


def case_34():
    s = Store()
    s.save(ORDERS + [order("D", "2026-03-01", "0.01", 7, "FR")])
    got = s.revenue_by_country()
    H.expect(got, [("DE", Decimal("2469.12")), ("NL", Decimal("10.29")), ("FR", Decimal("0.07"))], "exact, biggest first")


t.scenario("c34_revenue_by_country_sums_in_sql_exactly", case_34)


def case_35():
    path = os.path.join(tempfile.mkdtemp(prefix="pipeline-s6-"), "orders.sqlite")
    with Store(path) as s:
        s.save(ORDERS)
    try:
        s.count()
    except sqlite3.ProgrammingError:
        pass
    else:
        H.check(False, "the store should be closed after the with block")
    with Store(path) as again:
        H.expect(again.count(), 3, "the orders are still there when the file is opened again")


t.scenario("c35_a_store_is_a_context_manager_over_a_file", case_35)
t.finish()
