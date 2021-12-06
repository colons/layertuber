from typing import Tuple


def flip(y: float, x: float) -> Tuple[float, float]:
    return x, y


def px_to_center_offset(px: float, canvas: float) -> float:
    return (px / canvas) - 0.5


def px_to_center_offset_2d(px: Tuple[float, float], canvas: Tuple[float, float]) -> Tuple[float, float]:
    return px_to_center_offset(px[0], canvas[0]), px_to_center_offset(px[1], canvas[1])
