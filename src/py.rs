use std::thread;
use std::time::Duration;

pub fn run_tracker() {
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("from py host");
    }
}
