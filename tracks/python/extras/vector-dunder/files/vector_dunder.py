from __future__ import annotations


class Vector:
    """A 2D vector of ints that works with Python's operators and built-ins."""

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        """Vector(1, 2)"""
        raise NotImplementedError("TODO: implement Vector.__repr__")

    def __eq__(self, other: object) -> bool:
        """Equal coordinates; NotImplemented for anything that isn't a Vector."""
        raise NotImplementedError("TODO: implement Vector.__eq__")

    def __hash__(self) -> int:
        raise NotImplementedError("TODO: implement Vector.__hash__")

    def __add__(self, other: Vector) -> Vector:
        raise NotImplementedError("TODO: implement Vector.__add__")

    def __sub__(self, other: Vector) -> Vector:
        raise NotImplementedError("TODO: implement Vector.__sub__")

    def __mul__(self, k: int) -> Vector:
        """v * k for an int k."""
        raise NotImplementedError("TODO: implement Vector.__mul__")

    def __rmul__(self, k: int) -> Vector:
        """k * v for an int k."""
        raise NotImplementedError("TODO: implement Vector.__rmul__")

    def __abs__(self) -> float:
        """The length."""
        raise NotImplementedError("TODO: implement Vector.__abs__")

    def __bool__(self) -> bool:
        """False for the zero vector."""
        raise NotImplementedError("TODO: implement Vector.__bool__")

    def __iter__(self):
        """x, then y, so `x, y = v` works."""
        raise NotImplementedError("TODO: implement Vector.__iter__")
