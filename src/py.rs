use dirs::cache_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::mem::drop;
use std::os::unix::fs::PermissionsExt;
use std::thread;
use std::time::Duration;
use subprocess::{ExitStatus, Popen, PopenConfig, PopenError};

const TRACKER_BIN: &'static [u8] = include_bytes!("py/dist/layertuber");

enum RunTrackerError {
    Io(std::io::Error),
    Popen(PopenError),
}

impl From<std::io::Error> for RunTrackerError {
    fn from(err: std::io::Error) -> RunTrackerError {
        return RunTrackerError::Io(err);
    }
}

impl From<PopenError> for RunTrackerError {
    fn from(err: PopenError) -> RunTrackerError {
        return RunTrackerError::Popen(err);
    }
}

pub fn run_tracker() -> Result<(), RunTrackerError> {
    let cd = match cache_dir() {
        Some(p) => p,
        None => panic!("what's a cache directory"),
    };
    let tracker_bin_path = cd.join("layertuber-tracker");

    let mut tracker_bin = File::create(&tracker_bin_path)?;

    tracker_bin.write(TRACKER_BIN)?;
    tracker_bin.flush()?;

    let metadata = tracker_bin.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&tracker_bin_path, permissions)?;

    drop(tracker_bin);

    let cleanup = || {
        match fs::remove_file(&tracker_bin_path) {
            Ok(_) => eprintln!("deleted tracker"),
            Err(e) => eprintln!("tracker deletion failed: {}", e),
        };
    };

    let mut p = Popen::create(
        &[&tracker_bin_path],
        PopenConfig {
            // stdout: redirection::pipe,
            ..Default::default()
        },
    )?;

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
