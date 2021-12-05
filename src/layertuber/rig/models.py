from __future__ import annotations

from dataclasses import dataclass
from typing import List

from pyora import Project


class Rig:
    project: Project
    layers: List[Layer]
    groups: List[Group]

    def __init__(self, ora_path: str):
        raise NotImplementedError()


@dataclass
class Layer:
    image: bytes


@dataclass
class Group(Layer):
    layers: List[Layer]
