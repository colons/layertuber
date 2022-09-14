use crate::{puppet, tracker, Options};
use std::sync::mpsc::{channel, sync_channel};
use std::thread;

pub fn run_cli() {
    let options = Options::from_arguments();

    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    let tracker_options = options.clone();
    thread::spawn(move || {
        let tracker =
            tracker::run_tracker(control_rx, &tracker_options).expect("could not start tracker");
        for report in tracker {
            report_tx.send(report).unwrap()
        }
    });

    puppet::run_puppet(options.path.as_path(), report_rx, control_tx);
}
