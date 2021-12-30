from __future__ import annotations

from dataclasses import dataclass, field
from logging import getLogger
from queue import Queue
from typing import Optional, TYPE_CHECKING

import pygame

from ..rig import Rig
from ..tracking.face import CALIBRATE, NEXT_FRAME, TrackerControlEvent
from ..tracking.report import TrackingReport

if TYPE_CHECKING:
    from pygame.color import _ColorValue


logger = getLogger('viewer')


pygame.init()


@dataclass
class Viewer:
    rig: Rig
    reports: Queue[Optional[TrackingReport]]
    event_queue: Queue[TrackerControlEvent]
    background: _ColorValue = '#00ff00'

    screen: pygame.surface.Surface = field(init=False)

    def __post_init__(self) -> None:
        self.screen = pygame.display.set_mode(self.rig.target_size)
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
