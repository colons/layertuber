use crate::{puppet::Rig, tracker::ControlMessage};
use log::error;
use std::path::Path;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;

pub struct RenderThread {
    thread: thread::Thread,
    control_tx: Sender<ControlMessage>,
    frame_rx: Receiver<Box<[u8]>>,
}

pub fn render_thread(rig_path: &Path) -> RenderThread {
    let rig_path = rig_path.to_owned();

    let (frame_tx, frame_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    RenderThread {
        control_tx,
        frame_rx,
        thread: thread::spawn(move || {
            let rig = match Rig::open(rig_path.as_path()) {
                Ok(r) => r,
                Err(e) => {
                    error!("error loading rig: {}", e);
                    panic!("this should show an error message in OBS somehow");
                }
            };
            dbg!(&rig);
        })
        .thread()
        .to_owned(),
    }
}
