from __future__ import annotations

import sys
from dataclasses import dataclass, field
from logging import getLogger
from multiprocessing import Queue
from typing import Optional

import pygame

from ..rig import Rig
from ..tracking.face import CALIBRATE, NEXT_FRAME, TrackerControlEvent
from ..tracking.report import TrackingReport


logger = getLogger('viewer')


pygame.init()


@dataclass
class Viewer:
    rig: Rig
    reports: Queue[Optional[TrackingReport]]
    event_queue: Queue[TrackerControlEvent]
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
        self.event_queue.put(NEXT_FRAME)
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                elif event.type == pygame.KEYDOWN and event.key == pygame.K_b:
                    self.event_queue.put(CALIBRATE)

            self.screen.fill(pygame.color.Color(0, 255, 0))

            report = self.reports.get()
            self.event_queue.put(NEXT_FRAME)

            if report is not None:
                self.render(report)

            pygame.display.flip()
