use crate::puppet::ora;
use std::io;
use std::path::Path;

pub struct Rig {
    pub layers: Vec<u8>, // XXX some kind of texture from three-js?
}

impl Rig {
    pub fn open(ora_path: &Path) -> io::Result<Rig> {
        dbg!(&ora::layer_names(ora_path)?);

        Ok(Rig {
            layers: Vec::from([]),
        })
    }
}
