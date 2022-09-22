use crate::{
    options::Options,
    puppet::{render, Rig},
    tracker::{spawn_tracker, ControlMessage, TrackerOptions},
};
use log::error;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;
use three_d::{FrameInput, HeadlessContext, Viewport};

pub struct RenderThread {
    thread: thread::Thread,
    control_tx: Sender<ControlMessage>,
    frame_rx: Receiver<Box<[u8]>>,
}

pub fn render_thread(options: Options, width: u32, height: u32) -> RenderThread {
    let (frame_tx, frame_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    RenderThread {
        control_tx: control_tx.clone(),
        frame_rx,
        thread: thread::spawn(move || {
            let mut first_frame = false;
            let rig_path = options.path.as_path();

            let (report_rx, _thread) = spawn_tracker(
                TrackerOptions {
                    // XXX respect configuration
                    camera_index: 0,
                    show_features: false,
                },
                control_rx,
            );

            let context = HeadlessContext::new().unwrap();

            let rig = match Rig::open(rig_path) {
                Ok(r) => r,
                Err(e) => {
                    error!("error loading rig: {}", e);
                    panic!("this should show an error message in OBS somehow");
                }
            };
            let mut render_loop = render(&context, report_rx, control_tx, rig);
            loop {
                let input = FrameInput {
                    events: Vec::new(),
                    first_frame,
                    context: (*context).clone(),
                    viewport: Viewport {
                        x: 0,
                        y: 0,
                        width,
                        height,
                    },
                    window_width: width,
                    window_height: height,
                    device_pixel_ratio: 1.0,

                    // some naive defaults:
                    elapsed_time: 0.0,
                    accumulated_time: 0.0,
                };
                let output = render_loop(input);
                dbg!(&output);
                first_frame = false;
            }
        })
        .thread()
        .to_owned(),
    }
}
