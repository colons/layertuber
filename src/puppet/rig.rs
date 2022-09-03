use crate::puppet::ora;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use three_d_asset::io::RawAssets;
use three_d_asset::Texture2D;
use zip::read::ZipArchive;

#[derive(Debug)]
pub struct Rig {
    pub layers: Vec<Texture2D>,
}

impl Rig {
    pub fn open(ora_path: &Path) -> io::Result<Rig> {
        let mut ora = ZipArchive::new(File::open(ora_path)?)?;
        let mut layers = Vec::new();
        let mut assets = RawAssets::new();

        for path in ora::layer_names(&mut ora)? {
            let mut buf = Vec::new();
            ora.by_name(&path)?.read_to_end(&mut buf)?;
            assets.insert(&path, buf);
            layers.push(
                assets
                    .deserialize(&path)
                    .expect("this expect should be a question mark"),
            );
        }

        Ok(Rig { layers: layers })
    }
}
