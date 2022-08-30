use std::sync::mpsc::sync_channel;
use std::thread;

mod puppet;
mod tracker;

fn main() {
    let (tx, rx) = sync_channel(0);

    thread::spawn(|| {
        puppet::run_puppet(rx);
    });

    let tracker = tracker::run_tracker().expect("could not start tracker");
    for report in tracker {
        tx.send(report).unwrap()
    }
}
