from __future__ import annotations

from dataclasses import dataclass
from typing import List


@dataclass
class Rig:
    layers: List[Layer]
    groups: List[Group]


@dataclass
class Layer:
    image: bytes


@dataclass
class Group(Layer):
    layers: List[Layer]
