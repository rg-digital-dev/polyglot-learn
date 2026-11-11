# Part of the Deeper Python module (tracks/python/extras/timed-block). Don't
# edit this file: `learn` overwrites it. Your code goes in timed_block.py.

import os
import sys

# This file's folder (the exercise) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(os.path.dirname(DIR))]

from harness import H

import timed_block as M

t = H()

def case_01():
    log = []
    with M.tag(log, "p"):
        log.append("text")
    H.expect(log, ["<p>", "text", "</p>"], "log")


t.scenario("c01_tag_writes_opening_and_closing_tags", case_01)


def case_02():
    log = []
    with M.tag(log, "div"):
        with M.tag(log, "b"):
            log.append("x")
    H.expect(log, ["<div>", "<b>", "x", "</b>", "</div>"], "log")


t.scenario("c02_tags_nest", case_02)


def case_03():
    log = []
    try:
        with M.tag(log, "p"):
            raise KeyError("boom")
    except KeyError:
        pass
    else:
        H.check(False, "the KeyError should have propagated out of the with block")
    H.expect(log, ["<p>", "</p>"], "log")


t.scenario("c03_tag_closes_on_an_exception", case_03)


def case_04():
    log = []
    with M.Transaction(log):
        log.append("work")
    H.expect(log, ["begin", "work", "commit"], "log")


t.scenario("c04_a_transaction_commits_when_the_block_succeeds", case_04)


def case_05():
    log = []
    try:
        with M.Transaction(log):
            raise RuntimeError("disk full")
    except RuntimeError as e:
        H.expect(str(e), "disk full", "the exception that came out")
    else:
        H.check(False, "the RuntimeError should have propagated out of the with block")
    H.expect(log, ["begin", "rollback"], "log")


t.scenario("c05_a_transaction_rolls_back_and_lets_the_exception_through", case_05)


def case_06():
    log = []
    with M.Transaction(log) as tx:
        H.check(isinstance(tx, M.Transaction), f"`as` gave {tx!r}: return self from __enter__")
        tx.record("insert")
    H.expect(log, ["begin", "insert", "commit"], "log")


t.scenario("c06_with_gives_you_the_transaction", case_06)


def case_07():
    log = []
    with M.IgnoreValueErrors(log):
        int("not a number")
        log.append("never reached")
    H.expect(log, ["ignored: invalid literal for int() with base 10: 'not a number'"], "log")
    with M.IgnoreValueErrors(log):
        log.append("fine")
    H.expect(log[-1], "fine", "a block without an error")


t.scenario("c07_ignorevalueerrors_swallows_a_valueerror_and_logs_it", case_07)


def case_08():
    log = []
    try:
        with M.IgnoreValueErrors(log):
            raise TypeError("wrong type")
    except TypeError:
        pass
    else:
        H.check(False, "the TypeError should have propagated")
    H.expect(log, [], "log")


t.scenario("c08_ignorevalueerrors_lets_other_exceptions_through", case_08)
t.finish()
