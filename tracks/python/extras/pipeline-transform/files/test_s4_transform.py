# Stage 4 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in transform.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import itertools
from datetime import date, timedelta
from decimal import Decimal

from records import Order
from transform import batched, between, in_category, running_total

t = H()


def order(n, category="books", price="10.00", qty=1):
    return Order(f"O{n}", date(2026, 1, 1) + timedelta(days=n), "Ada", "Item", category, qty, Decimal(price), "NL")


ORDERS = [order(0), order(5, "games"), order(10), order(20, "toys"), order(31)]


def ids(orders):
    return [o.order_id for o in orders]


def case_19():
    H.expect(ids(between(ORDERS, date(2026, 1, 6), date(2026, 1, 21))), ["O5", "O10", "O20"], "both ends included")
    H.expect(ids(between(ORDERS, None, date(2026, 1, 11))), ["O0", "O5", "O10"], "no start")
    H.expect(ids(between(ORDERS, date(2026, 1, 21), None)), ["O20", "O31"], "no end")


t.scenario("c19_between_keeps_orders_in_the_inclusive_range", case_19)


def case_20():
    H.expect(ids(in_category(ORDERS, "GAMES", "toys")), ["O5", "O20"], "any of them, any case")
    H.expect(ids(in_category(ORDERS)), ids(ORDERS), "no categories keeps everything")


t.scenario("c20_in_category_matches_any_of_the_categories", case_20)


def case_21():
    H.expect(list(batched(range(7), 3)), [[0, 1, 2], [3, 4, 5], [6]], "batches of 3")
    H.expect(list(batched([], 3)), [], "nothing")
    H.expect(next(batched(itertools.count(), 2)), [0, 1], "lazy: works on an endless input")
    try:
        list(batched([1], 0))
    except ValueError:
        pass
    else:
        H.check(False, "a batch size of 0 should be a ValueError")


t.scenario("c21_batched_groups_into_lists", case_21)


def case_22():
    got = [(o.order_id, total) for o, total in running_total([order(1, price="2.50", qty=2), order(2, price="1.25"), order(3, price="0.05")])]
    H.expect(got, [("O1", Decimal("5.00")), ("O2", Decimal("6.25")), ("O3", Decimal("6.30"))], "order and total so far")


t.scenario("c22_running_total_pairs_each_order_with_the_total_so", case_22)


def case_23():
    endless = (order(n % 40, ("books", "toys")[n % 2]) for n in itertools.count())
    chain = in_category(between(endless, date(2026, 1, 5), None), "toys")
    first = list(itertools.islice(running_total(chain), 3))
    H.expect([o.order_id for o, _ in first], ["O5", "O7", "O9"], "the first three toys from the 5th on")
    H.expect(first[-1][1], Decimal("30.00"), "their running total")


t.scenario("c23_the_steps_are_lazy_and_chain", case_23)
t.finish()
