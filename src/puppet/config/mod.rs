pub use self::rules::Rule;
use self::rules::{FollowQuatRule, FollowVec2Rule, ThreeDimensions, ThresholdRule};
use crate::tracker::TrackingReport;
use core::ops::Mul;
use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::{io, io::Read};
use three_d::{Mat4, SquareMatrix};

mod rules;

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

    /// move absolutely in a direction
    pub follow: Option<FollowVec2Rule>,
}

impl LayerConfig {
    /// the transformation that this layer should have applied
    pub fn transform(&self, report: &TrackingReport) -> Mat4 {
        let mut transformation = match self.follow {
            Some(follow) => Mat4::from_translation(follow.apply(report)),
            None => Mat4::identity(),
        };

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
