/**
The types here must reflect the types defined in tracking/report.py
*/
use serde::Deserialize;
use three_d::Quaternion;

#[derive(Deserialize, Debug)]
pub struct TrackingReport {
    pub blink: f32,
    pub blink_left: f32,
    pub blink_right: f32,
    pub eyebrow_quirk: f32,
    pub eyebrow_quirk_left: f32,
    pub eyebrow_quirk_right: f32,
    pub eyebrow_steepness: f32,
    pub eyebrow_steepness_left: f32,
    pub eyebrow_steepness_right: f32,
    pub eyebrow_updown: f32,
    pub eyebrow_updown_left: f32,
    pub eyebrow_updown_right: f32,
    pub mouth_open: f32,
    pub mouth_wide: f32,

    pub head_rotation: [f32; 4],

    pub face_position: [f32; 2],
    pub left_gaze: [f32; 2],
    pub right_gaze: [f32; 2],
    pub gaze: [f32; 2],
}

pub trait Source<T> {
    fn value(&self, report: &TrackingReport) -> T;
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FloatSource {
    Blink,
    BlinkLeft,
    BlinkRight,
    EyebrowQuirk,
    EyebrowQuirkLeft,
    EyebrowQuirkRight,
    EyebrowSteepness,
    EyebrowSteepnessLeft,
    EyebrowSteepnessRight,
    EyebrowUpdown,
    EyebrowUpdownLeft,
    EyebrowUpdownRight,
    MouthOpen,
    MouthWide,
}

impl Source<f32> for FloatSource {
    fn value(&self, report: &TrackingReport) -> f32 {
        match self {
            FloatSource::Blink => report.blink,
            FloatSource::BlinkLeft => report.blink_left,
            FloatSource::BlinkRight => report.blink_right,
            FloatSource::EyebrowQuirk => report.eyebrow_quirk,
            FloatSource::EyebrowQuirkLeft => report.eyebrow_quirk_left,
            FloatSource::EyebrowQuirkRight => report.eyebrow_quirk_right,
            FloatSource::EyebrowSteepness => report.eyebrow_steepness,
            FloatSource::EyebrowSteepnessLeft => report.eyebrow_steepness_left,
            FloatSource::EyebrowSteepnessRight => report.eyebrow_steepness_right,
            FloatSource::EyebrowUpdown => report.eyebrow_updown,
            FloatSource::EyebrowUpdownLeft => report.eyebrow_updown_left,
            FloatSource::EyebrowUpdownRight => report.eyebrow_updown_right,
            FloatSource::MouthOpen => report.mouth_open,
            FloatSource::MouthWide => report.mouth_wide,
        }
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QuatSource {
    HeadRotation,
}

impl Source<Quaternion<f32>> for QuatSource {
    fn value(&self, report: &TrackingReport) -> Quaternion<f32> {
        let value = match self {
            QuatSource::HeadRotation => report.head_rotation,
        };

        Quaternion::new(
            // this ordering is weird because scipy deals in quats scalar-last, but three_d's are scalar-first
            value[3],
            // invert these to make the puppet behave like a mirror
            -value[0], // pitch
            -value[1], // yaw
            -value[2], // roll
        )
    }
}
