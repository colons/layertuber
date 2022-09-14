from math import floor
from typing import Tuple


def target_dimensions(max_dimensions: Tuple[int, int], source_dimensions: Tuple[int, int]) -> Tuple[int, int]:
    """
    Scale source_image_dimensions down to fit. Do not allow overflow, and do not scale up. Might underrun a little.

    >>> target_dimensions((800, 600), (1000, 1000))
    (600, 600)
    >>> target_dimensions((600, 800), (1000, 1000))
    (600, 600)
    >>> target_dimensions((1000, 1000), (250, 500))
    (250, 500)
    >>> target_dimensions((25, 20), (5, 25))
    (4, 20)
    >>> target_dimensions((20, 25), (25, 5))
    (20, 4)
    """

    if source_dimensions[0] <= max_dimensions[0] and source_dimensions[1] <= max_dimensions[1]:
        return source_dimensions

    source_aspect_ratio = source_dimensions[0] / source_dimensions[1]
    max_aspect_ratio = max_dimensions[0] / max_dimensions[1]

    if source_aspect_ratio > max_aspect_ratio:
        scaling_factor = max_dimensions[0] / source_dimensions[0]
    else:
        scaling_factor = max_dimensions[1] / source_dimensions[1]

    return (floor(source_dimensions[0] * scaling_factor), floor(source_dimensions[1] * scaling_factor))
