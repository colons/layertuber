use crate::{puppet, tracker, Options};
use std::sync::mpsc::channel;
use three_d::{Window, WindowSettings};

pub fn run_cli() {
    let options = Options::from_arguments();

    let (control_tx, control_rx) = channel();

    let window = Window::new(WindowSettings {
        title: "layertuber".to_string(),
        ..Default::default()
    })
    .unwrap();

    let (report_rx, _tracker_thread) = tracker::spawn_tracker(options.clone(), control_rx);

    let context = window.gl();

    let rig = puppet::Rig::open(options.path.as_path()).unwrap();

    window.render_loop(puppet::render(&context, report_rx, control_tx, rig))
}
