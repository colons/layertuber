from dataclasses import dataclass, field
from logging import getLogger

import pygame

from ..rig import Rig
from ..tracking.face import FaceTracker
from ..vendor.OpenSeeFace.tracker import FaceInfo


logger = getLogger('viewer')


pygame.init()


@dataclass
class Viewer:
    rig: Rig
    tracker: FaceTracker
    screen: pygame.surface.Surface = field(init=False)

    def __post_init__(self) -> None:
        self.screen = pygame.display.set_mode(self.rig.target_size)

    def render(self, face: FaceInfo) -> None:
        self.screen.blits([
            (layer.image, layer.position)
            for layer in self.rig.layers[::-1]
            if layer.visible
        ], False)

    def begin_loop(self) -> None:
        while self.tracker.reader.is_open():
            self.screen.fill(pygame.color.Color(0, 255, 0))

            face = self.tracker.get_face()
            if face is not None:
                self.render(face)

            pygame.display.flip()
