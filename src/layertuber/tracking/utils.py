from typing import Tuple


def flip(y: float, x: float) -> Tuple[float, float]:
    """
    >>> flip(1, 2)
    (2, 1)
    >>> flip(2, 1)
    (1, 2)
    """

    return x, y


def subtract(a: Tuple[float, float], b: Tuple[float, float]) -> Tuple[float, float]:
    """
    >>> subtract((3, 12), (1, 5))
    (2, 7)
    """

    return (a[0] - b[0], a[1] - b[1])


def px_to_center_offset(px: float, canvas: float) -> float:
    return (px / canvas) - 0.5


def px_to_center_offset_2d(px: Tuple[float, float], canvas: Tuple[float, float]) -> Tuple[float, float]:
    """
    >>> px_to_center_offset_2d((400, 300), (800, 600))
    (0.0, 0.0)
    >>> px_to_center_offset_2d((0, 0), (800, 600))
    (-0.5, -0.5)
    >>> px_to_center_offset_2d((640, 480), (640, 480))
    (0.5, 0.5)
    >>> px_to_center_offset_2d((40, 180), (320, 240))
    (-0.375, 0.25)
    """

    return px_to_center_offset(px[0], canvas[0]), px_to_center_offset(px[1], canvas[1])


def average_of_2d_vectors(*vecs: Tuple[float, float]) -> Tuple[float, float]:
    """
    >>> average_of_2d_vectors((0.1, 0.2))
    (0.1, 0.2)
    >>> average_of_2d_vectors((0.2, 0.4), (0.3, 0.6))
    (0.25, 0.5)
    >>> average_of_2d_vectors((0.2, 0.4), (0.3, 0.6), (0, 0), (0, 0))
    (0.125, 0.25)
    """

    return (
        sum(v[0] for v in vecs) / len(vecs),
        sum(v[1] for v in vecs) / len(vecs),
    )
