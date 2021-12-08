from typing import Dict, Literal, Tuple, TypedDict


FloatFromTrackingReport = Literal['left_blink', 'right_blink']
Vec2FromTrackingReport = Literal['face_position', 'left_gaze', 'right_gaze']


class TrackingReport(TypedDict):
    floats: Dict[FloatFromTrackingReport, float]
    vec2s: Dict[Vec2FromTrackingReport, Tuple[float, float]]
