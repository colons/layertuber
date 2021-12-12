import sys
from dataclasses import dataclass, field
from logging import getLogger

import pygame

from ..rig import Rig
from ..tracking.face import FaceTracker
from ..tracking.report import TrackingReport


logger = getLogger('viewer')


pygame.init()


@dataclass
class Viewer:
    rig: Rig
    tracker: FaceTracker
    screen: pygame.surface.Surface = field(init=False)

    def __post_init__(self) -> None:
        self.screen = pygame.display.set_mode(self.rig.target_size)
        pygame.display.set_caption('layertuber viewer')

    def render(self, report: TrackingReport) -> None:
        for group in self.rig.groups:
            group.update_from_report(report)

        for layer in self.rig.layers:
            layer.update_from_report(report)

        self.screen.blits([
            (layer.image, layer.position)
            for layer in self.rig.layers[::-1]
            if layer.visible
        ], False)

    def begin_loop(self) -> None:
        while self.tracker.reader.is_open():
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                elif event.type == pygame.KEYDOWN and event.key == pygame.K_b:
                    self.tracker.calibrate()

            self.screen.fill(pygame.color.Color(0, 255, 0))

            report = self.tracker.get_report()
            if report is not None:
                self.render(report)

            pygame.display.flip()
