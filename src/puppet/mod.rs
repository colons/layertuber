use crate::tracker::{ControlMessage, TrackingReport};
pub use rig::Rig;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use three_d::Context;
use three_d::{FrameInput, FrameOutput};

pub mod rig;

mod camera;
mod config;
mod conv;
mod ora;
mod render;

pub fn run_puppet(
    context: &Context,
    path: &Path,
    tracker_rx: Receiver<TrackingReport>,
    control_tx: Sender<ControlMessage>,
) -> Box<dyn FnMut(FrameInput) -> FrameOutput> {
    let rig = rig::Rig::open(path).unwrap();
    render::render(context, tracker_rx, control_tx, rig)
}
