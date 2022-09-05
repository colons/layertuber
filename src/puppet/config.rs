use crate::tracker::TrackingReport;
use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

trait Source<T> {
    fn value(&self, report: &TrackingReport) -> T;
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FloatSource {
    Blink,
}

impl Source<f32> for FloatSource {
    fn value(&self, report: &TrackingReport) -> f32 {
        match self {
            FloatSource::Blink => report.blink,
        }
    }
}

pub trait Rule<T> {
    fn apply(&self, report: &TrackingReport) -> T;
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct ThresholdRule {
    pub option: FloatSource,
    pub greater_than: f32,
}

impl Rule<bool> for ThresholdRule {
    fn apply(&self, report: &TrackingReport) -> bool {
        return self.option.value(report) > self.greater_than;
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct LayerConfig {
    #[serde(default = "default_visible")]
    pub visible: bool,

    pub visible_when: Option<ThresholdRule>,
    pub invisible_when: Option<ThresholdRule>,
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
