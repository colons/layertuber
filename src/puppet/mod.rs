use crate::tracker::{TrackingReport, ControlMessage};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

mod config;
mod conv;
mod ora;
mod render;
mod rig;

pub fn run_puppet(tracker_rx: Receiver<TrackingReport>, control_tx: Sender<ControlMessage>) {
    let rig = rig::Rig::open(&Path::new("examples/stick figure/stick figure.ora")).unwrap();
    render::render(tracker_rx, control_tx, rig);
}
