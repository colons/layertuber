from __future__ import annotations

from dataclasses import dataclass
from typing import List

import cv2

import numpy as np

from pyora import Project

from ..utils.cv import PINK, draw_dot_on_frame
from ..vendor.OpenSeeFace.tracker import FaceInfo


RENDER_WIDTH = 800
RENDER_HEIGHT = 600


class Rig:
    project: Project
    layers: List[Layer]
    groups: List[Group]

    def __init__(self, ora_path: str = ''):
        pass

    def render(self, face: FaceInfo) -> None:
        frame = np.empty((RENDER_HEIGHT, RENDER_WIDTH, 3), dtype=np.uint8)
        frame.fill(0)

        if face.eye_blink is not None:
            lblink, rblink = face.eye_blink
            draw_dot_on_frame(frame, PINK, 3, lblink * RENDER_WIDTH, 10)
            draw_dot_on_frame(frame, PINK, 3, rblink * RENDER_WIDTH, 20)

        cv2.imshow('rig', frame)
        cv2.waitKey(1)


@dataclass
class Layer:
    image: bytes


@dataclass
class Group(Layer):
    layers: List[Layer]
