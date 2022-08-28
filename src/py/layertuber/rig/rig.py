from __future__ import annotations

import logging
from typing import List, Tuple, Union

from pydantic import ValidationError

from pyora import Project
from pyora.Layer import Group as PyoraGroup, OpenRasterItemBase

import yaml

from .config import RigConfig
from .layer import LayerGroup, Renderable, from_layer
from .utils import target_dimensions


logger = logging.getLogger('rig')


class InvalidRig(BaseException):
    pass


class Rig:
    project: Project
    layers: List[Renderable]
    target_size: Tuple[int, int]
    minimum_dimension: int
    config: RigConfig

    def __init__(self, ora_path: str, max_size: Tuple[int, int]):
        try:
            self.project = Project.load(ora_path)
        except FileNotFoundError as e:
            raise InvalidRig(e)

        try:
            with open(f'{ora_path}.layertuber.yaml') as rig_config_file:
                self.config = RigConfig.parse_obj(yaml.load(rig_config_file, yaml.Loader))
        except (FileNotFoundError, ValidationError) as e:
            raise InvalidRig(e)

        self.layers = []

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

        missing_configured_layers = {configured_layer for configured_layer in self.config.layers.keys()} - {
            pyora_layer.name for pyora_layer in self.project.children_recursive
        }
        if missing_configured_layers:
            logger.warning(
                f'layers {", ".join((repr(n) for n in missing_configured_layers))} '
                'configured but do not exist in this image'
            )
