use std::thread;
use std::time::Duration;
use std::fs;
use std::mem::drop;
use std::os::unix::fs::PermissionsExt;
use std::fs::File;
use std::io::Write;
use dirs::cache_dir;
use subprocess::{ExitStatus, Popen, PopenConfig};

const TRACKER_BIN: &'static [u8] = include_bytes!("py/dist/layertuber");

pub fn run_tracker() {
    let cd = match cache_dir() {
        Some(p) => p,
        None => panic!("what's a cache directory"),
    };
    let tracker_bin_path = cd.join("layertuber-tracker");

    let mut tracker_bin = File::create(&tracker_bin_path).expect("failed to create tracker binary");

    tracker_bin.write(TRACKER_BIN).expect("failed to write tracker binary");
    tracker_bin.flush().expect("failed to flush tracker");

    let metadata = tracker_bin.metadata().expect("could not read metadata");
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o500);
    fs::set_permissions(&tracker_bin_path, permissions).expect("could not make tracker executable");

    drop(tracker_bin);

    let cleanup = || {
        match fs::remove_file(&tracker_bin_path) {
            Ok(_) => eprintln!("deleted tracker"),
            Err(e) => eprintln!("tracker deletion failed: {}", e),
        };
    };

    let mut p = match Popen::create(&[&tracker_bin_path], PopenConfig {
        // stdout: redirection::pipe,
        ..Default::default()
    }) {
        Ok(p) => p,
        Err(e) => {
            cleanup();
            panic!("failed to start tracker: {}", e)
        }
    };

    loop {
        match p.poll() {
            Some(e) => {
                cleanup();
                match e {
                    ExitStatus::Exited(s) => panic!("tracker died with exit code {}", s),
                    ExitStatus::Signaled(s) => panic!("tracker died with signal {}", s),
                    ExitStatus::Other(s) => panic!("tracker died for some reason: {}", s),
                    ExitStatus::Undetermined => panic!("tracker died for some reason"),
                }
            }
            None => (),
        };
        thread::sleep(Duration::from_secs(1));
        println!("from py host");
    }
}
