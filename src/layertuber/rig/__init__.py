from __future__ import annotations

import logging
from typing import List, Tuple

from PIL.Image import Image

from pygame.image import frombuffer
from pygame.sprite import Sprite
from pygame.surface import Surface

from pyora import Layer as PyoraLayer, Project, TYPE_LAYER

import yaml

from .config import LayerConfig, RigConfig
from .utils import target_dimensions
from ..tracking.model import TrackingReport


logger = logging.getLogger('rig')


class Rig:
    project: Project
    layers: List[Layer]
    target_size: Tuple[int, int]
    config: RigConfig

    def __init__(self, ora_path: str, max_size: Tuple[int, int]):
        with open(f'{ora_path}.layertuber.yaml') as rig_config_file:
            self.config = RigConfig.parse_obj(yaml.load(rig_config_file, yaml.Loader))

        self.project = Project.load(ora_path)
        self.layers = []

        seen_names = set()
        configured_layer_names = {layer_name for layer_name in self.config.layers.keys()}

        self.target_size = target_dimensions(max_size, self.project.dimensions)

        for pyora_layer in self.project.children_recursive:
            # we'll want this to retain heirarchy eventually, but for now:
            if pyora_layer.name in configured_layer_names:
                configured_layer_names.remove(pyora_layer.name)
            else:
                logger.info(f'layer {pyora_layer.name!r} has no configuration')

            if pyora_layer.type == TYPE_LAYER:
                layer = Layer(self, pyora_layer)
                if not layer.config.visible:
                    continue
                self.layers.append(layer)

            if pyora_layer.name in seen_names:
                raise RuntimeError(
                    f'this file has a duplicate layer named {pyora_layer.name!r}. '
                    'please rename your layers so that they are unique'
                )

            seen_names.add(layer.name)

        if configured_layer_names:
            logger.warning(
                f'layers {", ".join((repr(n) for n in configured_layer_names))} '
                'configured but do not exist in this image'
            )


class Layer(Sprite):
    image: Surface
    uuid: str
    name: str
    position: Tuple[float, float] = 0., 0.
    forced_invisible: bool = False
    visible: bool = True
    config: LayerConfig

    def __init__(self, rig: Rig, pyora_layer: PyoraLayer) -> None:
        pil_image: Image = pyora_layer.get_image_data(raw=False)
        pil_image = pil_image.resize(rig.target_size)
        self.image = frombuffer(pil_image.tobytes(), rig.target_size, 'RGBA')
        self.name = pyora_layer.name
        self.uuid = pyora_layer.uuid
        self.config = rig.config.layers.get(self.name) or LayerConfig()

    def update_from_report(self, report: TrackingReport) -> None:
        if self.config.visible_when is not None:
            self.visible = report[self.config.visible_when.option] > self.config.visible_when.greater_than

        if self.config.invisible_when is not None:
            self.visible = not (report[self.config.invisible_when.option] > self.config.invisible_when.greater_than)
