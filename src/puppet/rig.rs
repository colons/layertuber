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

    /// layer configurations, in the order they should be applied (starting from the root of the
    /// stack)
    pub configs: Vec<config::LayerConfig>,
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

        let config = config::load(ora_path)?;
        let (width, height, ora_layers) = ora::read(&mut ora)?;

        for ora_layer in ora_layers {
            let mut buf = Vec::new();
            ora.by_name(&ora_layer.src)?.read_to_end(&mut buf)?;
            assets.insert(&ora_layer.src, buf);

            let mut configs = Vec::new();

            for name in [ora_layer.parent_names, vec![*ora_layer.name]].concat() {
                if let Some(config) = config.layers.get(&name) {
                    configs.push(config.clone())
                }
            }

            layers.push(RigLayer {
                x: ora_layer.x,
                y: ora_layer.y,
                configs,
                name: ora_layer.name,
                texture: from_asset(
                    assets
                        .deserialize(&ora_layer.src)
                        .expect("this expect should be a question mark"),
                ),
            });
        }

        // XXX print out any unused config keys, since they're probably misconfigs

        Ok(Rig {
            width,
            height,
            layers,
        })
    }
}
