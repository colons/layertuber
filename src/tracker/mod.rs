use dirs::cache_dir;
use lazy_static::lazy_static;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::mem::drop;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use subprocess::{Communicator, ExitStatus, Popen, PopenConfig, PopenError, Redirection};
use report::TrackingReport;

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

pub struct FaceTracker {
    communicator: Communicator,
    cleanup: Box<dyn FnMut() -> ()>,
    p: Popen,
}

impl FaceTracker {
    fn new(mut p: Popen, cleanup: Box<dyn FnMut() -> ()>) -> FaceTracker {
        FaceTracker {
            communicator: p.communicate_start(None).limit_size(1),
            cleanup: cleanup,
            p: p,
        }
    }

    fn poll(&mut self) {
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
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();

        loop {
            match self.communicator.read_string() {
                Ok((out, _err)) => match out {
                    Some(out) => {
                        line.push_str(&out);
                        if out == "\n" {
                            break;
                        }
                    }
                    None => (),
                },
                Err(e) => {
                    (*self.cleanup)();
                    panic!("no tracking report: {}", e);
                }
            }
        }

        line
    }
}

impl Iterator for FaceTracker {
    type Item = TrackingReport;

    fn next(&mut self) -> Option<TrackingReport> {
        self.poll();
        let line = self.read_line();
        let report: TrackingReport = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => panic!("got bad data from tracker: {} ({})", line, e),
        };
        Some(report)
    }
}

pub fn run_tracker() -> Result<FaceTracker, RunTrackerError> {
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

    let tracker = FaceTracker::new(
        p,
        Box::new(|| {
            match fs::remove_file(TRACKER_BIN_PATH.as_path()) {
                Ok(_) => eprintln!("deleted tracker"),
                Err(e) => eprintln!("tracker deletion failed: {}", e),
            };
        }),
    );

    Ok(tracker)
}
