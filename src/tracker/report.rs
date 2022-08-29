/**
The types here must reflect the types defined in tracking/report.py
*/
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TrackingReport {
    pub blink_left: f32,
    pub blink_right: f32,
    pub blink: f32,
    pub eyebrow_quirk_left: f32,
    pub eyebrow_quirk_right: f32,
    pub eyebrow_quirk: f32,
    pub eyebrow_steepness_left: f32,
    pub eyebrow_steepness_right: f32,
    pub eyebrow_steepness: f32,
    pub eyebrow_updown_left: f32,
    pub eyebrow_updown_right: f32,
    pub eyebrow_updown: f32,
    pub mouth_open: f32,
    pub mouth_wide: f32,

    pub head_rotation: [f32; 4],

    pub face_position: [f32; 2],
    pub left_gaze: [f32; 2],
    pub right_gaze: [f32; 2],
    pub gaze: [f32; 2],
}
