def flip(y: float, x: float) -> tuple[float, float]:
    """
    >>> flip(1, 2)
    (2, 1)
    >>> flip(2, 1)
    (1, 2)
    """

    return x, y


def add(*vecs: tuple[float, float]) -> tuple[float, float]:
    """
    >>> add()
    (0, 0)
    >>> add((1, 1))
    (1, 1)
    >>> add((1, 1), (2, 3))
    (3, 4)
    >>> add((1, 1), (2, 3), (-1, -10))
    (2, -6)
    """

    return (
        sum(v[0] for v in vecs),
        sum(v[1] for v in vecs),
    )


def subtract(a: tuple[float, float], b: tuple[float, float]) -> tuple[float, float]:
    """
    >>> subtract((3, 12), (1, 5))
    (2, 7)
    """

    return (a[0] - b[0], a[1] - b[1])


def px_to_center_offset(px: float, canvas: float) -> float:
    return (px / canvas) - 0.5


def px_to_center_offset_2d(px: tuple[float, float], canvas: tuple[float, float]) -> tuple[float, float]:
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


def average_of_2d_vectors(*vecs: tuple[float, float]) -> tuple[float, float]:
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
