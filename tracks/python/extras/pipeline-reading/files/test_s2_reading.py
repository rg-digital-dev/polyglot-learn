# Stage 2 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in reading.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import inspect
import itertools

from reading import RowError, dedupe, read_csv, split_valid
from records import Order

t = H()

HEADER = "order_id,date,customer,product,category,quantity,unit_price,country\n"


def line(oid, qty="1", day="2026-01-05", product="Pen"):
    return f"{oid},{day},Ada,{product},stationery,{qty},2.50,NL\n"


def lines(*rows):
    return (HEADER + "".join(rows)).splitlines(keepends=True)


t.equals(
    "c07_read_csv_yields_orders_in_file_order",
    lambda: [o.order_id for o in read_csv(lines(line("A1"), line("A2"), line("A3")))],
    ["A1", "A2", "A3"],
)


def case_08():
    items = list(read_csv(lines(line("A1"), line("A2", qty="lots"), line("A3"))))
    H.expect(len(items), 3, "one item per row")
    H.check(isinstance(items[1], RowError), f"the bad row should be a RowError, got {items[1]!r}")
    H.expect(items[1].line, 3, "line number of the bad row (the header is line 1)")
    H.check("quantity" in items[1].message, f"the message {items[1].message!r} should say what's wrong")
    H.check(isinstance(items[2], Order), "reading goes on after a bad row")


t.scenario("c08_a_bad_row_is_a_rowerror_with_its_line_number", case_08)


def case_09():
    def endless():
        yield HEADER
        for i in itertools.count(1):
            yield line(f"A{i}")

    gen = read_csv(endless())
    H.check(inspect.isgenerator(gen), "read_csv should be a generator: use yield")
    H.expect([o.order_id for o in itertools.islice(gen, 2)], ["A1", "A2"], "the first two of an endless file")


t.scenario("c09_read_csv_is_lazy", case_09)


def case_10():
    text = lines('A1,2026-01-05,"Lovelace, Ada","Pen, blue",stationery,1,2.50,NL\n')
    o = next(read_csv(text))
    H.expect((o.customer, o.product), ("Lovelace, Ada", "Pen, blue"), "quoted fields")


t.scenario("c10_quoted_fields_with_commas_work", case_10)


def case_11():
    orders, errors = split_valid(read_csv(lines(line("A1"), line("A2", day="soon"), line("A3"), line("A4", qty="0"))))
    H.expect([o.order_id for o in orders], ["A1", "A3"], "orders")
    H.expect([e.line for e in errors], [3, 5], "error lines")
    H.check(isinstance(orders, list) and isinstance(errors, list), "split_valid returns two lists")


t.scenario("c11_split_valid_separates_orders_from_errors", case_11)


def case_12():
    orders = [o for o in read_csv(lines(line("A1", product="first"), line("A2"), line("A1", product="second")))]
    H.expect([(o.order_id, o.product) for o in dedupe(orders)], [("A1", "first"), ("A2", "Pen")], "first one wins")
    H.check(inspect.isgenerator(dedupe([])), "dedupe should be a generator")
    endless = (o for o in itertools.cycle(orders))
    H.expect(len(list(itertools.islice(dedupe(endless), 2))), 2, "dedupe works on an endless stream")


t.scenario("c12_dedupe_keeps_the_first_of_each_order_id_lazily", case_12)
t.finish()
