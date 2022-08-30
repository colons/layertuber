use std::path::Path;
use std::io;

pub struct Rig {
    pub layers: Vec<u8>,  // XXX some kind of texture from three-js?
}

impl Rig {
    pub fn open(ora_path: &Path) -> io::Result<Rig> {
        println!("{}", ora_path.display());
        Ok(Rig{
            layers: Vec::from([]),
        })
    }
}
