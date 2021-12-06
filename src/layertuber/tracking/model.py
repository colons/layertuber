from typing import Dict, Literal, TypedDict, Union


FloatFromTrackingReport = Union[Literal['left_blink', 'right_blink']]


class TrackingReport(TypedDict):
    floats: Dict[FloatFromTrackingReport, float]
