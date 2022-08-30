use crate::tracker::TrackingReport;
use std::sync::mpsc::Receiver;

pub fn run_puppet(rx: Receiver<TrackingReport>) {
    loop {
        dbg!(rx.recv().unwrap());
    }
}
