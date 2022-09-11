use crate::tracker::{FloatSource, QuatSource, Source, TrackingReport};
use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::ops::Mul;
use std::path::Path;
use std::{io, io::Read};
use three_d::{Mat4, Quaternion, SquareMatrix, Vec3};

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

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct ThreeDimensions {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl From<ThreeDimensions> for Vec3 {
    fn from(s: ThreeDimensions) -> Vec3 {
        Vec3 {
            x: s.x.unwrap_or(0.0),
            y: s.y.unwrap_or(0.0),
            z: s.z.unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct LayerConfig {
    #[serde(default = "default_visible")]
    pub visible: bool,

    /// become visible only when a source is above a certain amount
    pub visible_when: Option<ThresholdRule>,

    /// become invvisible only when a source is above a certain amount
    pub invisible_when: Option<ThresholdRule>,

    /// rotate in 3d with a source
    pub rotate_3d: Option<FollowQuatRule>,

    /// move absolutely in a direction
    pub offset: Option<ThreeDimensions>,
}

impl LayerConfig {
    /// the transformation that this layer should have applied
    pub fn transform(&self, report: &TrackingReport) -> Mat4 {
        let mut transformation = Mat4::identity();
        if let Some(rotate_3d) = self.rotate_3d {
            transformation = rotate_3d.apply(report).mul(transformation);
        }
        transformation
    }
}

impl Default for LayerConfig {
    fn default() -> LayerConfig {
        from_str("").unwrap()
    }
}

fn default_visible() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub layers: HashMap<String, LayerConfig>,
}

pub fn load(ora_path: &Path) -> io::Result<Config> {
    let mut config_string = String::new();
    let config_path = ora_path.with_file_name(format!(
        "{}.layertuber.yaml",
        match ora_path.file_name() {
            Some(f) => f.to_string_lossy(),
            None => panic!("no filename for {}", ora_path.display()),
        }
    ));
    let mut config_file = File::open(config_path)?;
    config_file.read_to_string(&mut config_string)?;
    Ok(from_str(&config_string).expect("this should be a question mark"))
}
