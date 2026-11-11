# Part of the Deeper Python module (tracks/python/extras/parse-commands). Don't
# edit this file: `learn` overwrites it. Your code goes in parse_commands.py.

import os
import sys

# This file's folder (the exercise) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(os.path.dirname(DIR))]

from harness import H

import parse_commands as M

t = H()

t.equals("c01_parse_reads_go_with_a_direction", lambda: [M.parse("go north"), M.parse("go west")], [M.Go("north"), M.Go("west")])
t.equals("c02_parse_joins_the_words_of_take", lambda: [M.parse("take lamp"), M.parse("take  the  old lamp")], [M.Take("lamp"), M.Take("the old lamp")])
t.equals("c03_parse_reads_drop_and_quit", lambda: [M.parse("drop key"), M.parse("quit"), M.parse("exit")], [M.Drop("key"), M.Quit(), M.Quit()])


def case_04():
    for line in ["go up", "go", "take", "drop a b", "dance", "", "quit now"]:
        try:
            got = M.parse(line)
        except NotImplementedError:
            raise
        except ValueError as e:
            H.expect(str(e), f"unknown command: {line}", f"the message for {line!r}")
        else:
            H.check(False, f"parse({line!r}) returned {got!r}; expected a ValueError")


t.scenario("c04_parse_rejects_anything_else", case_04)
t.equals(
    "c05_describe_matches_each_command_class",
    lambda: [M.describe(M.Go("north")), M.describe(M.Take("the old lamp")), M.describe(M.Drop("key")), M.describe(M.Quit())],
    ["you go north", "you take the old lamp", "you drop key", "bye"],
)
t.equals("c06_summarize_reads_a_login", lambda: M.summarize({"type": "login", "user": "ada", "ip": "10.0.0.1"}), "ada logged in")
t.equals(
    "c07_summarize_counts_purchased_items",
    lambda: [M.summarize({"type": "purchase", "user": "bo", "items": ["tea"]}), M.summarize({"type": "purchase", "user": "bo", "items": ["a", "b", "c"]})],
    ["bo bought 1 item", "bo bought 3 items"],
)
t.equals(
    "c08_summarize_uses_guards_on_error_codes",
    lambda: [M.summarize({"type": "error", "code": 503}), M.summarize({"type": "error", "code": 404})],
    ["server error 503", "client error 404"],
)


def case_09():
    H.expect(M.summarize({"type": "logout", "user": "ada"}), "unknown event logout", "a type without its own case")
    H.expect(M.summarize({"type": "error", "code": "500"}), "unknown event error", "an error whose code isn't an int")
    for bad in [{}, {"user": "ada"}, ["type", "login"], "login"]:
        try:
            M.summarize(bad)
        except NotImplementedError:
            raise
        except ValueError as e:
            H.expect(str(e), "bad event", f"the message for {bad!r}")
        else:
            H.check(False, f"summarize({bad!r}) should raise ValueError")


t.scenario("c09_summarize_handles_unknown_and_bad_events", case_09)
t.finish()
