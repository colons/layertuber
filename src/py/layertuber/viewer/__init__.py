from dataclasses import dataclass
from logging import getLogger
from queue import Queue
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
    screen: pygame.surface.Surface
    background: str = '#00ff00'

    def __post_init__(self) -> None:
        pygame.display.set_caption('layertuber viewer')

    def render(self, report: TrackingReport) -> None:
        for layer in self.rig.layers[::-1]:
            rendered = layer.render(report)
            if rendered is not None:
                self.screen.blit(rendered, (0, 0))

    def begin_loop(self) -> None:
        self.event_queue.put(NEXT_FRAME)
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_b:
                        self.event_queue.put(CALIBRATE)
                    elif event.key == pygame.K_q:
                        return

            report = self.reports.get()
            self.event_queue.put(NEXT_FRAME)

            if report is not None:
                self.screen.fill(pygame.color.Color(self.background))
                self.render(report)
                pygame.display.flip()
