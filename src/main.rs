use std::thread;

mod py;
mod puppet;

fn main() {
    thread::spawn(|| {
        py::run_tracker();
    });
    puppet::run_puppet()
}
