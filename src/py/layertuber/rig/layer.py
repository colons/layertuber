from abc import ABC, abstractmethod
from typing import List, Optional, TYPE_CHECKING, Tuple, Type, TypeVar

from PIL.Image import Image

from pygame import SRCALPHA
from pygame.image import frombuffer
from pygame.surface import Surface
from pygame.transform import rotozoom

from pyora import Layer as PyoraLayer, TYPE_GROUP, TYPE_LAYER

from .config import LayerConfig
from ..tracking.report import TrackingReport
from ..tracking.utils import add

if TYPE_CHECKING:
    from .rig import Rig


R = TypeVar('R', bound='Renderable')

FACING_POINT = (0, 0, 1)


class Renderable(ABC):
    uuid: str
    name: str
    position: Tuple[float, float] = (0., 0.)
    forced_invisible: bool = False
    visible: bool = True
    config: LayerConfig
    rig: Rig
    angle: float = 0  # in degrees, since that's what pygame uses

    @classmethod
    def from_layer(cls: Type[R], rig: Rig, pyora_layer: PyoraLayer) -> R:
        instance = cls()

        instance.rig = rig

        instance.name = pyora_layer.name
        instance.uuid = pyora_layer.uuid
        instance.config = rig.config.layers.get(instance.name) or LayerConfig()

        return instance

    @abstractmethod
    def untransformed_image(self, report: TrackingReport) -> Optional[Surface]:
        raise NotImplementedError()

    def currently_visible(self, report: TrackingReport) -> bool:
        if self.config.visible_when is not None:
            return report['floats'][self.config.visible_when.option] > self.config.visible_when.greater_than

        if self.config.invisible_when is not None:
            return report['floats'][self.config.invisible_when.option] <= self.config.invisible_when.greater_than

        else:
            return True

    def get_follow_offset(self, report: TrackingReport) -> Tuple[float, float]:
        if self.config.follow is not None:
            centre_offset = report['vec2s'][self.config.follow.option]
            return (
                centre_offset[0] * self.config.follow.scale * self.rig.minimum_dimension,
                centre_offset[1] * self.config.follow.scale * self.rig.minimum_dimension,
            )
        else:
            return (0, 0)

    def get_follow_xy_offset(self, report: TrackingReport) -> Tuple[float, float]:
        return (
            report['floats'][self.config.follow_x.option] * self.config.follow_x.scale * self.rig.minimum_dimension
            if self.config.follow_x is not None else 0,
            report['floats'][self.config.follow_y.option] * self.config.follow_y.scale * self.rig.minimum_dimension
            if self.config.follow_y is not None else 0,
        )

    def get_facing_point_offset(self, report: TrackingReport) -> Tuple[float, float]:
        if self.config.follow_facing_point is not None:
            rot = report['rotations'][self.config.follow_facing_point.option]
            x, y, z = rot.apply(FACING_POINT)
            return (
                (x * self.config.follow_facing_point.scale * self.rig.minimum_dimension),
                (y * self.config.follow_facing_point.scale * self.rig.minimum_dimension),
            )
        else:
            return (0, 0)

    def render(self, report: TrackingReport) -> Optional[Surface]:
        if not self.currently_visible(report):
            return None

        image = self.untransformed_image(report)

        if image is None:
            return None

        angle: float = 0
        position = add(
            self.get_follow_offset(report),
            self.get_facing_point_offset(report),
            self.get_follow_xy_offset(report),
        )

        if self.config.rotate_with is not None:
            angle += report['rotations'][self.config.rotate_with.option].as_rotvec(degrees=True)[2]

        surface = Surface(self.rig.target_size, SRCALPHA)

        if angle != 0:
            original_center = image.get_rect(topleft=position).center
            image = rotozoom(image, angle, 1)
            position = image.get_rect(center=original_center).topleft

        surface.blit(image, position)

        return surface


class LayerGroup(Renderable):
    layers: List[Renderable]

    def __init__(self) -> None:
        super().__init__()
        self.layers = []

    def untransformed_image(self, report: TrackingReport) -> Optional[Surface]:
        if not self.currently_visible(report):
            return None

        surface = Surface(self.rig.target_size, SRCALPHA)

        for layer in self.layers[::-1]:
            rendered = layer.render(report)
            if rendered is not None:
                surface.blit(rendered, (0, 0))

        return surface


class Layer(Renderable):
    original_image: Surface

    @classmethod
    def from_layer(cls, rig: Rig, pyora_layer: PyoraLayer) -> 'Layer':
        instance = super().from_layer(rig, pyora_layer)
        pil_image: Image = pyora_layer.get_image_data(raw=False)
        pil_image = pil_image.resize(rig.target_size)
        instance.original_image = frombuffer(pil_image.tobytes(), rig.target_size, 'RGBA').convert_alpha()
        return instance

    def untransformed_image(self, report: TrackingReport) -> Surface:
        return self.original_image


def from_layer(rig: Rig, pyora_layer: PyoraLayer) -> Renderable:
    if pyora_layer.type == TYPE_GROUP:
        return LayerGroup.from_layer(rig, pyora_layer)
    elif pyora_layer.type == TYPE_LAYER:
        return Layer.from_layer(rig, pyora_layer)
    else:
        raise TypeError(f'unrecognised pyora layer type: {pyora_layer.type}')
