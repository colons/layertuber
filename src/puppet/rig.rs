use crate::puppet::ora;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use three_d_asset::io::RawAssets;
use three_d_asset::Texture2D;
use zip::read::ZipArchive;

#[derive(Debug)]
pub struct RigLayer {
    pub texture: Texture2D,
    pub x: i32,
    pub y: i32,
    pub name: String,
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

        let (width, height, ora_layers) = ora::read(&mut ora)?;

        for ora_layer in ora_layers {
            let mut buf = Vec::new();
            ora.by_name(&ora_layer.src)?.read_to_end(&mut buf)?;
            assets.insert(&ora_layer.src, buf);
            layers.push(RigLayer {
                name: ora_layer.name,
                x: ora_layer.x,
                y: ora_layer.y,
                texture: assets
                    .deserialize(&ora_layer.src)
                    .expect("this expect should be a question mark"),
            });
        }

        Ok(Rig {
            width: width,
            height: height,
            layers: layers,
        })
    }
}
