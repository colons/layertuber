from __future__ import annotations

import logging
from typing import List, Tuple

from pyora import Project, TYPE_LAYER

import yaml

from .config import RigConfig
from .layer import Layer
from .utils import target_dimensions


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
