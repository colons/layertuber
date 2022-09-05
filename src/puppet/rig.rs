use crate::puppet::config;
use crate::puppet::conv::from_asset;
use crate::puppet::ora;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use three_d::CpuTexture;
use three_d_asset::io::RawAssets;
use zip::read::ZipArchive;

#[derive(Debug)]
pub struct RigLayer {
    pub texture: CpuTexture,
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub config: config::LayerConfig,
}

#[derive(Debug)]
pub struct Rig {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<RigLayer>,
}

impl Rig {
    pub fn open(ora_path: &Path) -> io::Result<Rig> {
        let mut ora = ZipArchive::new(File::open(ora_path)?)?;
        let mut layers = Vec::new();
        let mut assets = RawAssets::new();

        let mut config = config::load(ora_path)?;
        dbg!(&config);
        let (width, height, ora_layers) = ora::read(&mut ora)?;

        for ora_layer in ora_layers {
            let mut buf = Vec::new();
            ora.by_name(&ora_layer.src)?.read_to_end(&mut buf)?;
            assets.insert(&ora_layer.src, buf);
            layers.push(RigLayer {
                x: ora_layer.x,
                y: ora_layer.y,
                config: match config.layers.remove(&ora_layer.name) {
                    Some(c) => c,
                    None => {
                        eprintln!(
                            "{} present in image but not present in config file",
                            ora_layer.name
                        );
                        config::LayerConfig::default()
                    }
                },
                name: ora_layer.name,
                texture: from_asset(
                    assets
                        .deserialize(&ora_layer.src)
                        .expect("this expect should be a question mark"),
                ),
            });
        }

        // XXX print out the remaining config keys, since they're probably misconfigs

        Ok(Rig {
            width: width,
            height: height,
            layers: layers,
        })
    }
}
