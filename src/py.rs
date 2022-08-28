use std::thread;
use std::time::Duration;
use subprocess::{ExitStatus, Popen, PopenConfig};

const TRACKER_BIN: &'static [u8] = include_bytes!("py/dist/layertuber");

pub fn run_tracker() {
    let mut p = Popen::create(&["layertuber"], PopenConfig {
        // stdout: redirection::pipe,
        ..Default::default()
    }).expect("failed to start tracker");

    loop {
        match p.poll() {
            Some(ExitStatus::Exited(s)) => panic!("tracker died with exit code {}", s),
            Some(ExitStatus::Signaled(s)) => panic!("tracker died with signal {}", s),
            Some(ExitStatus::Other(s)) => panic!("tracker died for some reason: {}", s),
            Some(ExitStatus::Undetermined) => panic!("tracker died for some reason"),
            None => (),
        };
        thread::sleep(Duration::from_secs(1));
        println!("from py host");
    }
}
