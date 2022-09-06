use dirs::cache_dir;
use lazy_static::lazy_static;
pub use report::{FloatSource, QuatSource, Source, TrackingReport};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::drop;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use subprocess::{ExitStatus, Popen, PopenConfig, PopenError, Redirection};

const NEWLINE: u8 = "\n".as_bytes()[0];

lazy_static! {
    static ref TRACKER_BIN_PATH: PathBuf = cache_dir().unwrap().join("layertuber-tracker");
}

mod bin;
mod report;

pub enum ControlMessage {
    Calibrate,
}

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
    cleanup: Box<dyn FnMut() -> ()>,
    control_rx: Receiver<ControlMessage>,
    p: Popen,
}

impl FaceTracker {
    fn new(
        p: Popen,
        control_rx: Receiver<ControlMessage>,
        cleanup: Box<dyn FnMut() -> ()>,
    ) -> FaceTracker {
        FaceTracker {
            cleanup,
            control_rx,
            p,
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

    fn handle_input(&mut self) {
        loop {
            match self.control_rx.try_recv() {
                Ok(cm) => {
                    match cm {
                        ControlMessage::Calibrate => {
                            self.p.stdin.as_ref().unwrap().write("calibrate".as_bytes()).unwrap();
                        },
                    }
                },
                Err(_) => break,
            }
        }
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();

        loop {
            let mut one_char_buf: [u8; 1] = [0];

            match self.p.stdout.as_ref().unwrap().read(&mut one_char_buf) {
                Ok(size) => {
                    if size != 1 {
                        panic!("got {}-length read when line was: {}", size, line)
                    }
                    if one_char_buf == [NEWLINE] {
                        break;
                    }
                    line.push(one_char_buf[0] as char);
                }
                Err(e) => {
                    (*self.cleanup)();
                    panic!("no tracking report: {}", e);
                }
            }
        }

        self.p.stdin.as_ref().unwrap().write(&[NEWLINE]).unwrap();

        line
    }
}

impl Iterator for FaceTracker {
    type Item = TrackingReport;

    fn next(&mut self) -> Option<TrackingReport> {
        self.poll();
        self.handle_input();
        let line = self.read_line();

        let report: TrackingReport = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => panic!("got bad data from tracker: {} ({})", line, e),
        };
        Some(report)
    }
}

pub fn run_tracker<'a>(
    control_rx: Receiver<ControlMessage>,
) -> Result<FaceTracker, RunTrackerError> {
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
            stdin: Redirection::Pipe,
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let tracker = FaceTracker::new(
        p,
        control_rx,
        Box::new(|| {
            match fs::remove_file(TRACKER_BIN_PATH.as_path()) {
                Ok(_) => eprintln!("deleted tracker"),
                Err(e) => eprintln!("tracker deletion failed: {}", e),
            };
        }),
    );

    Ok(tracker)
}
