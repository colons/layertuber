use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct LayerConfig {
    pub visible: bool,
}

impl Default for LayerConfig {
    fn default() -> LayerConfig {
        LayerConfig { visible: true }
    }
}

// XXX this should return a hashmap keyed by name
pub fn load(ora_path: &Path) -> io::Result<LayerConfig> {
    let mut config_string = String::new();
    let config_path = ora_path.with_file_name(format!(
        "{}.layertuber.yaml",
        match ora_path.file_name() {
            Some(f) => f.to_string_lossy(),
            None => panic!("no filename for {}", ora_path.display()),
        }
    ));
    println!("{}", config_path.display());
    let mut config_file = File::open(config_path)?;
    config_file.read_to_string(&mut config_string)?;
    println!("{}", config_string);

    Ok(LayerConfig::default())
}
