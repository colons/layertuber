use crate::{puppet, tracker, Options};
use std::sync::mpsc::{channel, sync_channel};
use std::thread;
use three_d::{Window, WindowSettings};

pub fn run_cli() {
    let options = Options::from_arguments();

    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    let window = Window::new(WindowSettings {
        title: "layertuber".to_string(),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let tracker_options = options.clone();
    thread::spawn(move || {
        let tracker =
            tracker::run_tracker(control_rx, &tracker_options).expect("could not start tracker");
        for report in tracker {
            report_tx.send(report).unwrap()
        }
    });

    let rig = puppet::Rig::open(options.path.as_path()).unwrap();

    window.render_loop(puppet::render(&context, report_rx, control_tx, rig))
}
