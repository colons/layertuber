from typing import Dict, Literal, Tuple, TypedDict

from scipy.spatial.transform import Rotation


FloatFromTrackingReport = Literal['left_blink', 'right_blink']
RotationFromTrackingReport = Literal['head_rotation']
Vec2FromTrackingReport = Literal['face_position', 'left_gaze', 'right_gaze']


class TrackingReport(TypedDict):
    floats: Dict[FloatFromTrackingReport, float]
    rotations: Dict[RotationFromTrackingReport, Rotation]
    vec2s: Dict[Vec2FromTrackingReport, Tuple[float, float]]
