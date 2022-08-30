use crate::tracker::TrackingReport;
use std::sync::mpsc::Receiver;
use three_d::window::{Window, WindowSettings};

pub fn run_puppet(rx: Receiver<TrackingReport>) {
    let window = Window::new(WindowSettings::default()).unwrap();
    dbg!(window.size());

    loop {
        dbg!(rx.recv().unwrap());
    }
}
