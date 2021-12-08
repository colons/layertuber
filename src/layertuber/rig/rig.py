from __future__ import annotations

import logging
from typing import Dict, List, Tuple

from pyora import Project, TYPE_GROUP, TYPE_LAYER

import yaml

from .config import RigConfig
from .layer import Layer, LayerGroup
from .utils import target_dimensions


logger = logging.getLogger('rig')


class Rig:
    project: Project
    layers: List[Layer]
    groups: List[LayerGroup]
    target_size: Tuple[int, int]
    minimum_dimension: int
    config: RigConfig

    def __init__(self, ora_path: str, max_size: Tuple[int, int]):
        with open(f'{ora_path}.layertuber.yaml') as rig_config_file:
            self.config = RigConfig.parse_obj(yaml.load(rig_config_file, yaml.Loader))

        self.project = Project.load(ora_path)
        self.layers = []
        self.groups = []

        configured_layer_names = {layer_name for layer_name in self.config.layers.keys()}

        self.target_size = target_dimensions(max_size, self.project.dimensions)
        self.minimum_dimension = min(self.target_size)
        groups_by_uuid: Dict[str, LayerGroup] = {}

        # make groups
        for pyora_layer in self.project.children_recursive:
            if pyora_layer.type == TYPE_GROUP:
                group = LayerGroup.from_layer(self, pyora_layer)
                if not group.config.visible:
                    continue
                self.groups.append(group)
                groups_by_uuid[group.uuid] = group

        # make layers
        for pyora_layer in self.project.children_recursive:
            if pyora_layer.name in configured_layer_names:
                configured_layer_names.remove(pyora_layer.name)
            else:
                logger.info(f'layer {pyora_layer.name!r} has no configuration')

            if pyora_layer.type == TYPE_LAYER:
                layer = Layer.from_layer(self, pyora_layer)
                if not layer.config.visible:
                    continue
                self.layers.append(layer)
                parent = pyora_layer.parent
                while parent is not None:
                    layer_group = groups_by_uuid.get(parent.uuid)
                    if layer_group is not None:
                        layer.add(layer_group)
                    parent = parent.parent

        if configured_layer_names:
            logger.warning(
                f'layers {", ".join((repr(n) for n in configured_layer_names))} '
                'configured but do not exist in this image'
            )
