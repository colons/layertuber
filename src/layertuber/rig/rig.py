from __future__ import annotations

import logging
from typing import List, Tuple, Union

from pyora import Project
from pyora.Layer import Group as PyoraGroup, OpenRasterItemBase

import yaml

from .config import RigConfig
from .layer import LayerGroup, Renderable, from_layer
from .utils import target_dimensions


logger = logging.getLogger('rig')


class Rig:
    project: Project
    layers: List[Renderable]
    target_size: Tuple[int, int]
    minimum_dimension: int
    config: RigConfig

    def __init__(self, ora_path: str, max_size: Tuple[int, int]):
        with open(f'{ora_path}.layertuber.yaml') as rig_config_file:
            self.config = RigConfig.parse_obj(yaml.load(rig_config_file, yaml.Loader))

        self.project = Project.load(ora_path)
        self.layers = []

        configured_layer_names = {layer_name for layer_name in self.config.layers.keys()}

        self.target_size = target_dimensions(max_size, self.project.dimensions)
        self.minimum_dimension = min(self.target_size)

        def add_layer(parent: Union[Rig, LayerGroup], pyora_layer: OpenRasterItemBase) -> None:
            layer = from_layer(self, pyora_layer)
            if not layer.config.visible:
                return
            parent.layers.append(layer)
            if isinstance(pyora_layer, PyoraGroup):
                for child in pyora_layer.children:
                    assert isinstance(layer, LayerGroup)
                    add_layer(layer, child)

        # make groups
        for pyora_layer in self.project.children:
            add_layer(self, pyora_layer)

        if configured_layer_names:
            logger.warning(
                f'layers {", ".join((repr(n) for n in configured_layer_names))} '
                'configured but do not exist in this image'
            )
