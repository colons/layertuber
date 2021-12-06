from typing import Dict, Literal, Union


FloatFromTrackingReport = Union[Literal["left_blink", "right_blink"]]


class TrackingReport(Dict[FloatFromTrackingReport, float]):
    pass
