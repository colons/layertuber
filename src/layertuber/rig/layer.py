from __future__ import annotations

from abc import ABC
from typing import TYPE_CHECKING, Tuple, Type, TypeVar

from PIL.Image import Image

from pygame.image import frombuffer
from pygame.sprite import Group, Sprite
from pygame.surface import Surface
from pygame.transform import rotate

from pyora import Layer as PyoraLayer

from .config import LayerConfig
from ..tracking.report import TrackingReport

if TYPE_CHECKING:
    from .rig import Rig


C = TypeVar('C', bound='ConfigurableThing')

FACING_POINT = (0, 0, 1)


class ConfigurableThing(ABC):
    uuid: str
    name: str
    position: Tuple[float, float] = (0., 0.)
    forced_invisible: bool = False
    visible: bool = True
    config: LayerConfig
    rig: Rig
    angle: float = 0  # in degrees, since that's what pygame uses

    @classmethod
    def from_layer(cls: Type[C], rig: Rig, pyora_layer: PyoraLayer) -> C:
        instance = cls()

        instance.rig = rig

        instance.name = pyora_layer.name
        instance.uuid = pyora_layer.uuid
        instance.config = rig.config.layers.get(instance.name) or LayerConfig()

        return instance

    def update_visibility(self, report: TrackingReport) -> None:
        if self.config.visible_when is not None:
            self.visible = (
                report['floats'][self.config.visible_when.option] > self.config.visible_when.greater_than
            )

        if self.config.invisible_when is not None:
            self.visible = not (
                report['floats'][self.config.invisible_when.option] > self.config.invisible_when.greater_than
            )

    def update_position(self, report: TrackingReport) -> None:
        position = (0., 0.)
        angle: float = 0

        if isinstance(self, Layer):
            for group in self.groups():
                if isinstance(group, LayerGroup):
                    position = (position[0] + group.position[0], position[1] + group.position[1])

        if self.config.follow is not None:
            centre_offset = report['vec2s'][self.config.follow.option]
            our_px = (
                centre_offset[0] * self.config.follow.scale * self.rig.minimum_dimension,
                centre_offset[1] * self.config.follow.scale * self.rig.minimum_dimension,
            )
            position = (position[0] + our_px[0], position[1] + our_px[1])

        if self.config.follow_facing_point is not None:
            rot = report['rotations'][self.config.follow_facing_point.option]
            x, y, z = rot.apply(FACING_POINT)
            position = (
                position[0] + (x * self.config.follow_facing_point.scale * self.rig.minimum_dimension),
                position[1] + (y * self.config.follow_facing_point.scale * self.rig.minimum_dimension),
            )

        if self.config.rotate_with is not None:
            angle += report['rotations'][self.config.rotate_with.option].as_rotvec(degrees=True)[2]

        if isinstance(self, Layer):
            angle = angle + sum((g.angle for g in self.groups() if isinstance(g, LayerGroup)))
            if angle != 0:
                self.image = rotate(self.image, angle)

        self.angle = angle
        self.position = position

    def update_from_report(self, report: TrackingReport) -> None:
        self.update_visibility(report)
        self.update_position(report)


class LayerGroup(Group, ConfigurableThing):
    pass


class Layer(Sprite, ConfigurableThing):
    original_image: Surface
    image: Surface

    @classmethod
    def from_layer(cls, rig: Rig, pyora_layer: PyoraLayer) -> Layer:
        instance = super().from_layer(rig, pyora_layer)
        pil_image: Image = pyora_layer.get_image_data(raw=False)
        pil_image = pil_image.resize(rig.target_size)
        instance.original_image = frombuffer(pil_image.tobytes(), rig.target_size, 'RGBA')
        return instance

    def update_position(self, report: TrackingReport) -> None:
        self.image = self.original_image
        super().update_position(report)
