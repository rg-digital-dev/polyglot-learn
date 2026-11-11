# Stage 3 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in sources.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import tempfile
from decimal import Decimal

from reading import RowError
from records import Order
from sources import CsvSource, JsonLinesSource, Source, merged, open_source

t = H()

TMP = tempfile.mkdtemp(prefix="pipeline-s3-")
CSV = "order_id,date,customer,product,category,quantity,unit_price,country\nC1,2026-02-01,Ada,Pen,stationery,2,1.50,NL\nC2,2026-02-02,Bob,Ink,stationery,1,4.00,DE\n"
JSONL = (
    '{"order_id": "J1", "date": "2026-02-03", "customer": "Cy", "product": "Lamp", "category": "home", "quantity": 1, "unit_price": 24.5, "country": "fr"}\n'
    "\n"
    "not json\n"
    '["a", "list"]\n'
    '{"order_id": "J2", "date": "2026-02-04", "customer": "Di", "product": "Mug", "category": "home", "quantity": 2, "unit_price": "6.00", "country": "BE"}\n'
)


def file(name, text):
    path = os.path.join(TMP, name)
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(text)
    return path


def case_13():
    src = CsvSource(file("shop.csv", CSV))
    H.expect(src.name, "shop.csv", "a source's name is its file name")
    H.expect([o.order_id for o in src.records()], ["C1", "C2"], "orders")


t.scenario("c13_csvsource_reads_a_file", case_13)


def case_14():
    items = list(JsonLinesSource(file("web.jsonl", JSONL)).records())
    orders = [i for i in items if isinstance(i, Order)]
    H.expect([o.order_id for o in orders], ["J1", "J2"], "orders")
    H.expect(orders[0].unit_price, Decimal("24.5"), "a JSON number becomes a Decimal")
    H.expect((orders[0].quantity, orders[0].country), (1, "FR"), "quantity and country")


t.scenario("c14_jsonlinessource_reads_numbers_and_strings", case_14)


def case_15():
    errors = [i for i in JsonLinesSource(file("web2.jsonl", JSONL)).records() if isinstance(i, RowError)]
    H.expect([e.line for e in errors], [3, 4], "the bad lines, counting the blank one")
    H.check(all(e.message for e in errors), "every error has a message")


t.scenario("c15_a_bad_json_line_is_a_rowerror_with_its_line_numb", case_15)


def case_16():
    H.check(isinstance(open_source("x/a.csv"), CsvSource), "a .csv file is a CsvSource")
    H.check(isinstance(open_source("b.JSONL"), JsonLinesSource), "a .jsonl file is a JsonLinesSource")
    H.check(isinstance(open_source("c.ndjson"), JsonLinesSource), "a .ndjson file is a JsonLinesSource")
    try:
        open_source("d.xlsx")
    except ValueError as e:
        H.check(".xlsx" in str(e), f"the error {str(e)!r} should name the extension")
    else:
        H.check(False, "an .xlsx file was accepted")


t.scenario("c16_open_source_picks_by_extension", case_16)


class Fake:
    """Not a subclass of anything: it only has the right shape."""

    name = "fake"

    def records(self):
        yield RowError(1, "made up")


def case_17():
    H.check(isinstance(CsvSource("a.csv"), Source), "CsvSource fits the Source protocol")
    H.check(isinstance(JsonLinesSource("a.jsonl"), Source), "JsonLinesSource fits the Source protocol")
    H.check(isinstance(Fake(), Source), "anything with name and records() is a Source (make it @runtime_checkable)")
    H.check(not isinstance(object(), Source), "an object without them isn't")


t.scenario("c17_anything_with_name_and_records_is_a_source", case_17)


def case_18():
    sources = [CsvSource(file("one.csv", CSV)), Fake()]
    got = [(name, getattr(item, "order_id", None) or item.message) for name, item in merged(sources)]
    H.expect(got, [("one.csv", "C1"), ("one.csv", "C2"), ("fake", "made up")], "tagged items, source by source")


t.scenario("c18_merged_tags_each_item_with_its_source", case_18)
t.finish()
