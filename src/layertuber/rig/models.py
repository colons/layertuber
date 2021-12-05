from __future__ import annotations

from dataclasses import dataclass


@dataclass
class Rig:
    layers: List[Layer]
    groups: List[Group]


@dataclass
class Layer:
    image: bin


@dataclass
class Group(Layer):
    layers: List[Layer]
