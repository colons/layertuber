use crate::tracker::{FloatSource, QuatSource, Source, TrackingReport, Vec2Source};
use serde::Deserialize;
use three_d::{Mat4, Quaternion, Vec3};

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct ThreeDimensions {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl From<ThreeDimensions> for Vec3 {
    fn from(s: ThreeDimensions) -> Self {
        Vec3 {
            x: s.x.unwrap_or(0.0),
            y: s.y.unwrap_or(0.0),
            z: s.z.unwrap_or(0.0),
        }
    }
}

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
pub struct FollowVec2Rule {
    source: Vec2Source,
    scale: ThreeDimensions,
}

impl Rule<Vec3> for FollowVec2Rule {
    fn apply(&self, report: &TrackingReport) -> Vec3 {
        let value = self.source.value(report);
        return Vec3 {
            x: value.x * Vec3::from(self.scale).x,
            y: value.y * Vec3::from(self.scale).y,
            z: 0.0,
        };
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
