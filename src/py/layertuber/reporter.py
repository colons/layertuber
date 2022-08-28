from __future__ import annotations

from dataclasses import dataclass
from queue import Queue
from typing import Optional

import orjson

from scipy.spatial.transform import Rotation

from .tracking.face import NEXT_FRAME, TrackerControlEvent
from .tracking.report import TrackingReport


def default(o: object) -> object:
    if isinstance(o, Rotation):
        return o.as_quat()

    raise TypeError()


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
                print(orjson.dumps(report, default=default, option=orjson.OPT_SERIALIZE_NUMPY))
