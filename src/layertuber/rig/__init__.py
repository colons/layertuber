from __future__ import annotations

# import os
from typing import List, Tuple

from PIL.Image import Image

from pygame.image import frombuffer
from pygame.sprite import Sprite
from pygame.surface import Surface

from pyora import Layer as PyoraLayer, Project, TYPE_LAYER

from .utils import target_dimensions
from ..tracking.model import TrackingReport


class Rig:
    project: Project
    layers: List[Layer]
    target_size: Tuple[int, int]
    # config: ConfigParser

    def __init__(self, ora_path: str, max_size: Tuple[int, int]):
        # this should probably be yaml and maybe camel; this structure sucks and section names are case-insensitive
        # self.config = ConfigParser()
        # self.config.read(f"{os.path.splitext(ora_path)[0]}.layertuber.ini")

        self.project = Project.load(ora_path)
        self.layers = []

        seen_names = set()

        self.target_size = target_dimensions(max_size, self.project.dimensions)

        for layer in self.project.children_recursive:
            # we'll want this to retain heirarchy eventually, but for now:
            if layer.type == TYPE_LAYER:
                self.layers.append(Layer(self, layer))

            if layer.name in seen_names:
                raise RuntimeError(
                    f'this file has a duplicate layer named {layer.name!r}. '
                    'please rename your layers so that they are unique'
                )
            seen_names.add(layer.name)


class Layer(Sprite):
    image: Surface
    uuid: str
    name: str
    position: Tuple[float, float] = 0., 0.
    forced_invisible: bool = False
    visible: bool = True

    def __init__(self, rig: Rig, pyora_layer: PyoraLayer) -> None:
        pil_image: Image = pyora_layer.get_image_data(raw=False)
        pil_image = pil_image.resize(rig.target_size)
        self.image = frombuffer(pil_image.tobytes(), rig.target_size, 'RGBA')
        self.name = pyora_layer.name
        self.uuid = pyora_layer.uuid

    def update_from_report(self, report: TrackingReport) -> None:
        left_eye_open = report['left_blink'] > 0.8
        right_eye_open = report['right_blink'] > 0.8

        if self.name in ('eye l open', 'pupil l'):
            self.visible = left_eye_open
        elif self.name in ('eye r open', 'pupil r'):
            self.visible = right_eye_open
        elif self.name in ('eye r closed',):
            self.visible = not right_eye_open
        elif self.name in ('eye l closed',):
            self.visible = not left_eye_open
