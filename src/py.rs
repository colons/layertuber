use std::thread;
use std::time::Duration;
use subprocess::{Popen, PopenConfig};

pub fn run_tracker() {
    let p = Popen::create(&["layertuber"], PopenConfig {
        // stdout: Redirection::Pipe,
        ..Default::default()
    });
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("from py host");
    }
}
