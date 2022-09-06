use std::sync::mpsc::{channel, sync_channel};
use std::thread;

mod puppet;
mod tracker;

fn main() {
    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    thread::spawn(move || {
        let tracker = tracker::run_tracker(control_rx).expect("could not start tracker");
        for report in tracker {
            report_tx.send(report).unwrap()
        }
    });

    puppet::run_puppet(report_rx, control_tx);
}
