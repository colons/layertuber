use crate::{puppet::{render, Rig}, tracker::ControlMessage};
use log::error;
use std::path::Path;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;
use three_d::{Window, WindowSettings};

pub struct RenderThread {
    thread: thread::Thread,
    control_tx: Sender<ControlMessage>,
    frame_rx: Receiver<Box<[u8]>>,
}

pub fn render_thread(rig_path: &Path) -> RenderThread {
    let rig_path = rig_path.to_owned();

    let (frame_tx, frame_rx) = sync_channel(0);
    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    RenderThread {
        control_tx: control_tx.clone(),
        frame_rx,
        thread: thread::spawn(move || {
            // XXX this should be off-screen, not in a window
            let window = Window::new(WindowSettings::default()).unwrap();
            let context = window.gl();

            let rig = match Rig::open(rig_path.as_path()) {
                Ok(r) => r,
                Err(e) => {
                    error!("error loading rig: {}", e);
                    panic!("this should show an error message in OBS somehow");
                }
            };
            let render_loop = render(&context, report_rx, control_tx, rig);
            window.render_loop(render_loop);
        })
        .thread()
        .to_owned(),
    }
}
