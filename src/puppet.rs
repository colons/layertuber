use crate::tracker::TrackingReport;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub fn run_puppet(rx: Receiver<TrackingReport>) {
    loop {
        thread::sleep(Duration::from_secs(1));
        dbg!(rx.recv().unwrap());
    }
}
