from __future__ import annotations

from dataclasses import dataclass
from queue import Queue
from typing import Optional

from .tracking.face import NEXT_FRAME, TrackerControlEvent
from .tracking.report import TrackingReport


@dataclass
class Reporter:
    reports: Queue[Optional[TrackingReport]]

    # XXX this should go away, and the rust host should be telling us when to get new frames:
    event_queue: Queue[TrackerControlEvent]

    def begin_loop(self) -> None:
        self.event_queue.put(NEXT_FRAME)

        while True:
            report = self.reports.get()
            self.event_queue.put(NEXT_FRAME)

            if report is not None:
                print(report)
