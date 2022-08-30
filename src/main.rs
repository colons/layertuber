use std::thread;

mod puppet;
mod tracker;

fn main() {
    thread::spawn(|| {
        puppet::run_puppet();
    });

    let tracker = tracker::run_tracker().expect("could not start tracker");
    for report in tracker {
        dbg!(report);
    }
}
