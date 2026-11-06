# Part of the Deeper Python module (tracks/python/extras/lazy-pipeline). Don't
# edit this file: `learn` overwrites it. Your code goes in lazy_pipeline.py.

import os
import sys

# This file's folder (the exercise) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import lazy_pipeline as M

t = H()

import inspect
import time


def counting(n):
    """A generator of 1..n that records how many items were pulled from it."""
    pulled = []

    def gen():
        for i in range(1, n + 1):
            pulled.append(i)
            yield i

    return gen(), pulled


t.equals("c01_countdown_yields_from_n_down_to_1", lambda: (list(M.countdown(3)), list(M.countdown(0))), ([3, 2, 1], []))


def case_02():
    start = time.perf_counter()
    first = next(M.countdown(10**12))
    H.expect(first, 10**12, "first item")
    H.check(time.perf_counter() - start < 0.5, "countdown built every item before yielding the first one")


t.scenario("c02_countdown_is_lazy", case_02)
t.equals("c03_take_stops_an_endless_generator", lambda: M.take(5, M.naturals()), [1, 2, 3, 4, 5])


def case_04():
    source, pulled = counting(100)
    H.expect(M.take(3, source), [1, 2, 3], "result")
    H.expect(pulled, [1, 2, 3], "items pulled from the source")
    H.expect(M.take(0, M.naturals()), [], "take(0)")


t.scenario("c04_take_pulls_only_what_it_needs", case_04)
t.equals("c05_chunks_groups_items_with_a_shorter_last_chunk", lambda: list(M.chunks([1, 2, 3, 4, 5], 2)), [[1, 2], [3, 4], [5]])
t.equals("c06_chunks_works_on_an_endless_generator", lambda: M.take(2, M.chunks(M.naturals(), 3)), [[1, 2, 3], [4, 5, 6]])
t.equals("c07_flatten_walks_nested_lists_in_order", lambda: list(M.flatten([[1, [2, 3]], 4, [[5]], []])), [1, 2, 3, 4, 5])
t.equals("c08_flatten_of_nothing_is_empty", lambda: list(M.flatten([])), [])


def case_09():
    for name, gen in [("countdown", M.countdown(2)), ("naturals", M.naturals()), ("chunks", M.chunks([1], 1)), ("flatten", M.flatten([1]))]:
        H.check(inspect.isgenerator(gen), f"{name} returned {type(gen).__name__}, not a generator: use yield")


t.scenario("c09_every_function_is_a_generator", case_09)
t.finish()
