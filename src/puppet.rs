use std::thread;
use std::time::Duration;

pub fn run_puppet() {
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("from puppet");
    }
}
