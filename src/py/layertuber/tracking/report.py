from typing import Literal, TypedDict

from scipy.spatial.transform import Rotation


FloatFromTrackingReport = Literal[
    'blink_left', 'blink_right', 'blink',
    'eyebrow_quirk_left', 'eyebrow_quirk_right', 'eyebrow_quirk',
    'eyebrow_steepness_left', 'eyebrow_steepness_right', 'eyebrow_steepness',
    'eyebrow_updown_left', 'eyebrow_updown_right', 'eyebrow_updown',
    'mouth_open', 'mouth_wide',
]
RotationFromTrackingReport = Literal['head_rotation']
Vec2FromTrackingReport = Literal['face_position', 'left_gaze', 'right_gaze', 'gaze']


class TrackingReport(TypedDict):
    floats: dict[FloatFromTrackingReport, float]
    rotations: dict[RotationFromTrackingReport, Rotation]
    vec2s: dict[Vec2FromTrackingReport, tuple[float, float]]
