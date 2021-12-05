from typing import Tuple

import numpy


def draw_dot_on_frame(frame: numpy.ndarray, colour: Tuple[int, int, int], size: int, x: int, y: int) -> None:
    width, height, _depth = frame.shape
    padding = tuple(range(0 - size, 1 + size))

    for xm, ym in [(xm, ym) for xm in padding for ym in padding]:
        part_x = int(x + xm + 0.5)
        part_y = int(y + ym + 0.5)
        if not (x < 0 or y < 0 or x >= height or y >= width):
            frame[part_x, part_y] = colour
