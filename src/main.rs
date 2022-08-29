use std::thread;

mod puppet;
mod py;

fn main() {
    thread::spawn(|| {
        py::run_tracker().expect("could not start tracker");
    });
    puppet::run_puppet()
}
