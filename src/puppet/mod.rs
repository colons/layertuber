use crate::tracker::{ControlMessage, TrackingReport};
pub use rig::Rig;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

pub mod rig;

mod camera;
mod config;
mod conv;
mod ora;
mod render;

pub fn run_puppet(
    path: &Path,
    tracker_rx: Receiver<TrackingReport>,
    control_tx: Sender<ControlMessage>,
) {
    let rig = rig::Rig::open(path).unwrap();
    render::render(tracker_rx, control_tx, rig);
}
