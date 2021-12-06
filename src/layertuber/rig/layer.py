from __future__ import annotations

from typing import TYPE_CHECKING, Tuple

from PIL.Image import Image

from pygame.image import frombuffer
from pygame.sprite import Sprite
from pygame.surface import Surface

from pyora import Layer as PyoraLayer

from .config import LayerConfig
from ..tracking.model import TrackingReport

if TYPE_CHECKING:
    from .rig import Rig


class Layer(Sprite):
    image: Surface
    uuid: str
    name: str
    position: Tuple[float, float] = 0., 0.
    forced_invisible: bool = False
    visible: bool = True
    config: LayerConfig
    rig: Rig

    def __init__(self, rig: Rig, pyora_layer: PyoraLayer) -> None:
        self.rig = rig

        pil_image: Image = pyora_layer.get_image_data(raw=False)
        pil_image = pil_image.resize(self.rig.target_size)
        self.image = frombuffer(pil_image.tobytes(), rig.target_size, 'RGBA')

        self.name = pyora_layer.name
        self.uuid = pyora_layer.uuid
        self.config = rig.config.layers.get(self.name) or LayerConfig()

    def update_from_report(self, report: TrackingReport) -> None:
        if self.config.visible_when is not None:
            self.visible = (
                report['floats'][self.config.visible_when.option] > self.config.visible_when.greater_than
            )

        if self.config.invisible_when is not None:
            self.visible = not (
                report['floats'][self.config.invisible_when.option] > self.config.invisible_when.greater_than
            )

        if self.config.follow is not None:
            centre_offset = report['vec2s'][self.config.follow.option]
            self.position = (
                centre_offset[0] * self.config.follow.scale * self.rig.target_size[0],
                centre_offset[1] * self.config.follow.scale * self.rig.target_size[1],
            )
