use std::thread;
use std::time::Duration;
use std::fs;
use std::mem::drop;
use std::os::unix::fs::PermissionsExt;
use std::io::Write;
use subprocess::{ExitStatus, Popen, PopenConfig};
use tempfile::NamedTempFile;

const TRACKER_BIN: &'static [u8] = include_bytes!("py/dist/layertuber");

pub fn run_tracker() {
    let mut tracker_bin = NamedTempFile::new().expect("could not make temporary file for tracker");
    tracker_bin.write(TRACKER_BIN).expect("failed to write tracker binary");
    tracker_bin.flush().expect("failed to flush tracker");

    let (file, path) = tracker_bin.keep().expect("failed to persist tracker");

    let metadata = file.metadata().expect("could not read metadata");
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o500);
    fs::set_permissions(&path, permissions).expect("could not make tracker executable");

    drop(file);

    let cleanup = || {
        match fs::remove_file(&path) {
            Ok(_) => eprintln!("deleted tracker"),
            Err(e) => eprintln!("tracker deletion failed: {}", e),
        };
    };

    let mut p = match Popen::create(&[&path], PopenConfig {
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
