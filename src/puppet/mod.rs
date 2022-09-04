use crate::tracker::TrackingReport;
use std::path::Path;
use std::sync::mpsc::Receiver;

mod conv;
mod ora;
mod render;
mod rig;

pub fn run_puppet(rx: Receiver<TrackingReport>) {
    let rig = rig::Rig::open(&Path::new("examples/stick figure/stick figure.ora")).unwrap();
    render::render(rx, rig);
}
