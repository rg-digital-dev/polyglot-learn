# Part of the Deeper Python module (tracks/python/extras/retry-decorator). Don't
# edit this file: `learn` overwrites it. Your code goes in retry_decorator.py.

import os
import sys

# This file's folder (the exercise) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import retry_decorator as M

t = H()

def make_flaky(failures):
    """A function that raises `failures` times, then returns "ok"; it records its attempts."""
    attempts = []

    def flaky():
        attempts.append(len(attempts) + 1)
        if len(attempts) <= failures:
            raise ConnectionError(f"attempt {len(attempts)} failed")
        return "ok"

    return flaky, attempts


def case_01():
    @M.shout
    def greet(name):
        return f"hi {name}"

    H.expect(greet("ada"), "HI ADA", "result")


t.scenario("c01_shout_uppercases_the_result", case_01)


def case_02():
    @M.shout
    def greet(name):
        """Say hi."""
        return f"hi {name}"

    H.expect(greet.__name__, "greet", "__name__")
    H.expect(greet.__doc__, "Say hi.", "__doc__")


t.scenario("c02_shout_keeps_the_function_s_name_and_docstring", case_02)


def case_03():
    @M.count_calls
    def ping():
        return "pong"

    H.expect(ping.calls, 0, "calls before any call")
    ping()
    ping()
    H.expect(ping.calls, 2, "calls after two calls")


t.scenario("c03_count_calls_counts_every_call", case_03)


def case_04():
    @M.count_calls
    def add(a, b, *, scale=1):
        return (a + b) * scale

    H.expect(add(2, 3), 5, "add(2, 3)")
    H.expect(add(2, 3, scale=10), 50, "add(2, 3, scale=10)")
    H.expect(add.__name__, "add", "__name__")


t.scenario("c04_count_calls_passes_arguments_and_results_through", case_04)


def case_05():
    flaky, attempts = make_flaky(2)
    H.expect(M.retry(3)(flaky)(), "ok", "result")
    H.expect(attempts, [1, 2, 3], "attempts")


t.scenario("c05_retry_returns_the_first_success", case_05)


def case_06():
    flaky, attempts = make_flaky(5)
    try:
        M.retry(3)(flaky)()
    except ConnectionError as e:
        H.expect(str(e), "attempt 3 failed", "the exception raised (the last one)")
    else:
        H.check(False, "expected the last ConnectionError to be raised")
    H.expect(attempts, [1, 2, 3], "attempts")


t.scenario("c06_retry_gives_up_after_the_given_number_of_attempts", case_06)


def case_07():
    flaky, attempts = make_flaky(0)
    H.expect(M.retry(5)(flaky)(), "ok", "result")
    H.expect(attempts, [1], "attempts")


t.scenario("c07_retry_doesn_t_call_a_working_function_twice", case_07)


def case_08():
    @M.shout
    @M.count_calls
    def name():
        return "ada"

    H.expect(name(), "ADA", "result")
    H.expect(name.__wrapped__.calls, 1, "the inner wrapper's calls (reached through functools.wraps)")


t.scenario("c08_decorators_stack", case_08)
t.finish()
