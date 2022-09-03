use crate::puppet::ora;
use std::fs::File;
use std::io;
use std::path::Path;
use three_d::core::texture::Texture2D;
use zip::read::ZipArchive;

pub struct Rig {
    pub layers: Vec<Texture2D>,
}

impl Rig {
    pub fn open(ora_path: &Path) -> io::Result<Rig> {
        let mut ora = ZipArchive::new(File::open(ora_path)?)?;

        for path in ora::layer_names(&mut ora)? {
            dbg!(ora.by_name(&path)?.size());
        }

        Ok(Rig {
            layers: Vec::from([]),
        })
    }
}
