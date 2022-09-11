use crate::tracker::{ControlMessage, TrackingReport};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

mod camera;
mod config;
mod conv;
mod ora;
mod render;
mod rig;

pub fn run_puppet(
    path: &Path,
    tracker_rx: Receiver<TrackingReport>,
    control_tx: Sender<ControlMessage>,
) {
    let rig = rig::Rig::open(path).unwrap();
    render::render(tracker_rx, control_tx, rig);
}
