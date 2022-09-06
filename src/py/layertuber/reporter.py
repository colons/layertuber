from dataclasses import dataclass
from sys import stdout
from typing import Literal, Optional

import orjson

from scipy.spatial.transform import Rotation

from .tracking.report import TrackingReport


REPORT_TYPES: tuple[Literal['floats', 'rotations', 'vec2s'], ...] = ('floats', 'rotations', 'vec2s')


def default(o: object) -> object:
    if isinstance(o, Rotation):
        return o.as_quat()

    raise TypeError()


@dataclass
class Reporter:
    def report(self, report: Optional[TrackingReport]) -> None:
        if report is not None:
            flattened_report = {
                k: v for t in REPORT_TYPES for k, v in report[t].items()
            }

            stdout.buffer.write(orjson.dumps(flattened_report, default=default, option=orjson.OPT_SERIALIZE_NUMPY))
            stdout.buffer.write(b'\n')
