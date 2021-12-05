from __future__ import annotations

from typing import List, Tuple

from PIL.Image import Image

from pygame.image import frombuffer
from pygame.sprite import Sprite
from pygame.surface import Surface

from pyora import Layer as PyoraLayer, Project, TYPE_LAYER

from .utils import target_dimensions


class Rig:
    project: Project
    layers: List[Layer]
    target_size: Tuple[int, int]

    def __init__(self, ora_path: str, max_size: Tuple[int, int]):
        self.project = Project.load(ora_path)
        self.layers = []
        self.target_size = target_dimensions(max_size, self.project.dimensions)

        for layer in self.project.children_recursive:
            # we'll want this to retain heirarchy eventually, but for now:
            if layer.type == TYPE_LAYER:
                self.layers.append(Layer(self, layer))


class Layer(Sprite):
    image: Surface
    uuid: str
    name: str
    position: Tuple[float, float] = 0., 0.

    def __init__(self, rig: Rig, pyora_layer: PyoraLayer) -> None:
        pil_image: Image = pyora_layer.get_image_data(raw=False)
        pil_image = pil_image.resize(rig.target_size)
        self.image = frombuffer(pil_image.tobytes(), rig.target_size, 'RGBA')
        self.name = pyora_layer.name
        self.uuid = pyora_layer.uuid

        print(f"{self.uuid} {self.name}")
