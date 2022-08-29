use std::thread;

mod puppet;
mod tracker;

fn main() {
    thread::spawn(|| {
        tracker::run_tracker().expect("could not start tracker");
    });
    puppet::run_puppet()
}
