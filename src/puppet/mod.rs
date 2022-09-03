use crate::tracker::TrackingReport;
use std::path::Path;
use std::sync::mpsc::Receiver;

mod ora;
mod render;
mod rig;

pub fn run_puppet(rx: Receiver<TrackingReport>) {
    let rig = rig::Rig::open(&Path::new("examples/demo/demo.ora")).unwrap();
    dbg!(&rig);
    render::render(rx, rig);
}
