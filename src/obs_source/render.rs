use crate::{
    options::Options,
    puppet::{render, Rig},
    tracker::{spawn_tracker, TrackerOptions},
};
use log::error;
use std::sync::mpsc::channel;
use three_d::{Context, FrameInput, FrameOutput};

pub fn create_renderer(
    context: Context,
    options: Options,
) -> Box<dyn FnMut(FrameInput) -> FrameOutput> {
    let (control_tx, control_rx) = channel();

    let rig_path = options.path.as_path();

    let (report_rx, _thread) = spawn_tracker(TrackerOptions::from(&options), control_rx);

    let rig = match Rig::open(rig_path) {
        Ok(r) => r,
        Err(e) => {
            error!("error loading rig: {}", e);
            panic!("this should show an error message in OBS somehow");
        }
    };

    render(context, report_rx, control_tx, rig)
}
