from dataclasses import dataclass
from sys import stdout
from typing import Optional

import orjson

from scipy.spatial.transform import Rotation

from .tracking.report import TrackingReport


def default(o: object) -> object:
    if isinstance(o, Rotation):
        return o.as_quat()

    raise TypeError()


@dataclass
class Reporter:
    def report(self, report: Optional[TrackingReport]) -> None:
        if report is not None:
            stdout.buffer.write(orjson.dumps(report, default=default, option=orjson.OPT_SERIALIZE_NUMPY))
            stdout.buffer.write(b'\n')
