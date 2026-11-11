# Stage 1 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in records.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import dataclasses
from datetime import date
from decimal import Decimal

from records import Order, parse_order

t = H()

ROW = {
    "order_id": "A-1001",
    "date": "2026-03-14",
    "customer": "Ada",
    "product": "Notebook",
    "category": "Stationery",
    "quantity": "3",
    "unit_price": "19.99",
    "country": "nl",
}


def row(**changes):
    return {**ROW, **changes}


def case_01():
    want = Order("A-1001", date(2026, 3, 14), "Ada", "Notebook", "stationery", 3, Decimal("19.99"), "NL")
    H.expect(parse_order(ROW), want, "the order")


t.scenario("c01_parses_a_valid_row", case_01)


def case_02():
    o = parse_order(row(customer="  Grace  ", category=" BOOKS ", country=" de ", quantity=" 2 "))
    H.expect((o.customer, o.category, o.country, o.quantity), ("Grace", "books", "DE", 2), "customer, category, country, quantity")
    H.check(isinstance(o.unit_price, Decimal), f"unit_price should be a Decimal, not {type(o.unit_price).__name__}")
    H.check(isinstance(o.date, date), f"date should be a date, not {type(o.date).__name__}")
    H.expect(parse_order({**ROW, "extra": "ignored"}).order_id, "A-1001", "extra fields are ignored")


t.scenario("c02_strips_spaces_and_normalises_category_and_countr", case_02)


def case_03():
    H.expect(parse_order(ROW).total, Decimal("59.97"), "3 × 19.99, exactly")
    H.expect(parse_order(row(quantity="10", unit_price="0.10")).total, Decimal("1.00"), "10 × 0.10")


t.scenario("c03_total_is_quantity_times_unit_price", case_03)


def case_04():
    for field in ("order_id", "date", "customer", "product", "category", "quantity", "unit_price", "country"):
        for broken in ({k: v for k, v in ROW.items() if k != field}, row(**{field: "   "})):
            try:
                parse_order(broken)
            except ValueError as e:
                H.check(field in str(e), f"the error {str(e)!r} should name {field!r}")
            else:
                H.check(False, f"a row without {field} was accepted")


t.scenario("c04_a_missing_or_blank_field_is_an_error_naming_it", case_04)


def case_05():
    bad = {
        "quantity": ["two", "0", "-1", "1.5"],
        "unit_price": ["abc", "-1", "NaN"],
        "date": ["2026-13-01", "14/03/2026"],
        "country": ["NLD", "1A"],
    }
    for field, values in bad.items():
        for value in values:
            try:
                parse_order(row(**{field: value}))
            except ValueError as e:
                H.check(field in str(e), f"the error for {field}={value!r} should name {field!r}: {str(e)!r}")
            else:
                H.check(False, f"{field}={value!r} was accepted")
    H.expect(parse_order(row(unit_price="0")).unit_price, Decimal("0"), "a free item is fine")


t.scenario("c05_bad_numbers_dates_and_countries_are_errors_namin", case_05)


def case_06():
    o = parse_order(ROW)
    try:
        o.quantity = 5
    except dataclasses.FrozenInstanceError:
        pass
    else:
        H.check(False, "changing a field worked: make the dataclass frozen")
    H.expect(len({o, parse_order(ROW)}), 1, "equal orders in a set")


t.scenario("c06_orders_are_frozen_and_hashable", case_06)
t.finish()
