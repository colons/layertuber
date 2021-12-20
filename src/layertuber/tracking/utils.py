from typing import Tuple


def flip(y: float, x: float) -> Tuple[float, float]:
    return x, y


def subtract(a: Tuple[float, float], b: Tuple[float, float]) -> Tuple[float, float]:
    return (a[0] - b[0], a[1] - b[1])


def px_to_center_offset(px: float, canvas: float) -> float:
    return (px / canvas) - 0.5


def px_to_center_offset_2d(px: Tuple[float, float], canvas: Tuple[float, float]) -> Tuple[float, float]:
    return px_to_center_offset(px[0], canvas[0]), px_to_center_offset(px[1], canvas[1])


def average_of_2d_vectors(*vecs: Tuple[float, float]) -> Tuple[float, float]:
    return (
        sum(v[0] for v in vecs) / len(vecs),
        sum(v[1] for v in vecs) / len(vecs),
    )
