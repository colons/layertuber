use crate::tracker::{FloatSource, QuatSource, Source, TrackingReport};
use serde::Deserialize;
use three_d::{Mat4, Quaternion, Vec3};

const IDENTITY_QUAT: Quaternion<f32> = Quaternion {
    v: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    s: 1.0,
};

pub trait Rule<T> {
    fn apply(&self, report: &TrackingReport) -> T;
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct ThresholdRule {
    source: FloatSource,
    greater_than: f32,
}

impl Rule<bool> for ThresholdRule {
    fn apply(&self, report: &TrackingReport) -> bool {
        self.source.value(report) > self.greater_than
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct FollowQuatRule {
    source: QuatSource,
    scale: f32,
}

impl Rule<Mat4> for FollowQuatRule {
    fn apply(&self, report: &TrackingReport) -> Mat4 {
        let quat = self.source.value(report);
        IDENTITY_QUAT.slerp(quat, self.scale).into()
    }
}
