# Part of the Deeper Python module (tracks/python/extras/vector-dunder). Don't
# edit this file: `learn` overwrites it. Your code goes in vector_dunder.py.

import os
import sys

# This file's folder (the exercise) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import vector_dunder as M

t = H()

V = M.Vector

t.equals("c01_repr_shows_the_coordinates", lambda: [repr(V(1, 2)), str([V(0, -3)])], ["Vector(1, 2)", "[Vector(0, -3)]"])


def case_02():
    H.check(V(1, 2) == V(1, 2), "Vector(1, 2) == Vector(1, 2) should be True")
    H.check(V(1, 2) != V(2, 1), "Vector(1, 2) != Vector(2, 1) should be True")
    H.check(V(1, 2) != (1, 2), "a Vector shouldn't equal a tuple")
    H.check(V(0, 0) != None, "a Vector shouldn't equal None")


t.scenario("c02_equal_vectors_compare_equal", case_02)


def case_03():
    seen = {V(1, 2): "a"}
    seen[V(1, 2)] = "b"
    H.expect(len(seen), 1, "keys after adding an equal vector")
    H.expect(len({V(0, 0), V(0, 0), V(1, 0)}), 2, "set size")


t.scenario("c03_vectors_work_as_dict_keys", case_03)
t.equals("c04_plus_and_minus_add_and_subtract", lambda: [V(1, 2) + V(10, 20), V(5, 5) - V(1, 2)], [V(11, 22), V(4, 3)])
t.equals("c05_times_scales_by_a_number_on_either_side", lambda: [V(1, -2) * 3, 3 * V(1, -2), V(4, 4) * 0], [V(3, -6), V(3, -6), V(0, 0)])
t.equals("c06_abs_gives_the_length", lambda: [abs(V(3, 4)), abs(V(0, 0))], [5.0, 0.0])


def case_07():
    H.check(not V(0, 0), "Vector(0, 0) should be falsy")
    H.check(bool(V(0, 1)), "Vector(0, 1) should be truthy")


t.scenario("c07_a_zero_vector_is_falsy", case_07)


def case_08():
    x, y = V(7, 8)
    H.expect((x, y), (7, 8), "x, y = Vector(7, 8)")
    H.expect(list(V(1, 2)), [1, 2], "list(Vector(1, 2))")


t.scenario("c08_vectors_unpack_into_x_and_y", case_08)


def case_09():
    for label, op in [("v + 1", lambda: V(1, 2) + 1), ("v * v", lambda: V(1, 2) * V(1, 2)), ("'a' * v", lambda: "a" * V(1, 2)), ("v - None", lambda: V(1, 2) - None)]:
        try:
            got = op()
        except NotImplementedError:
            raise
        except TypeError:
            continue
        H.check(False, f"{label} returned {got!r}; return NotImplemented so Python raises TypeError")


t.scenario("c09_unsupported_operands_raise_typeerror", case_09)
t.finish()
