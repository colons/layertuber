use dirs::cache_dir;
use lazy_static::lazy_static;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::mem::drop;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use subprocess::{ExitStatus, Popen, PopenConfig, PopenError, Redirection};

lazy_static! {
    static ref TRACKER_BIN_PATH: PathBuf = cache_dir().unwrap().join("layertuber-tracker");
}

mod bin;
mod report;

#[derive(Debug)]
pub enum RunTrackerError {
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

struct RunningTracker {
    p: Popen,
    cleanup: Box<dyn FnMut() -> ()>,
}

impl RunningTracker {
    pub fn new(p: Popen, cleanup: Box<dyn FnMut() -> ()>) -> RunningTracker {
        RunningTracker {
            p: p,
            cleanup: cleanup,
        }
    }

    pub fn begin(&mut self) {
        loop {
            match self.p.poll() {
                Some(e) => {
                    (*self.cleanup)();
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
}

pub fn run_tracker() -> Result<(), RunTrackerError> {
    let mut tracker_bin = File::create(TRACKER_BIN_PATH.as_path())?;

    tracker_bin.write(bin::TRACKER_BIN)?;
    tracker_bin.flush()?;

    let metadata = tracker_bin.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(TRACKER_BIN_PATH.as_path(), permissions)?;

    drop(tracker_bin);

    let p = Popen::create(
        &[TRACKER_BIN_PATH.as_path()],
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let mut tracker = RunningTracker::new(
        p,
        Box::new(|| {
            match fs::remove_file(TRACKER_BIN_PATH.as_path()) {
                Ok(_) => eprintln!("deleted tracker"),
                Err(e) => eprintln!("tracker deletion failed: {}", e),
            };
        }),
    );
    tracker.begin();

    Ok(())
}
