from typing import Dict, Literal, Tuple, TypedDict, Union


FloatFromTrackingReport = Union[Literal['left_blink', 'right_blink']]
Vec2FromTrackingReport = Union[Literal['face_position']]


class TrackingReport(TypedDict):
    floats: Dict[FloatFromTrackingReport, float]
    vec2s: Dict[Vec2FromTrackingReport, Tuple[float, float]]
