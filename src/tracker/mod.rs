use crate::Options;
use dirs::cache_dir;
use lazy_static::lazy_static;
use log::{error, info};
pub use report::{FloatSource, QuatSource, Source, TrackingReport, Vec2Source};
use std::ffi::OsStr;
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
    Die,
}

#[derive(Debug)]
pub enum RunTrackerError {
    Io(std::io::Error),
    Popen(PopenError),
}

impl From<std::io::Error> for RunTrackerError {
    fn from(err: std::io::Error) -> RunTrackerError {
        RunTrackerError::Io(err)
    }
}

impl From<PopenError> for RunTrackerError {
    fn from(err: PopenError) -> RunTrackerError {
        RunTrackerError::Popen(err)
    }
}

pub struct FaceTracker {
    cleanup: Box<dyn FnMut()>,
    control_rx: Receiver<ControlMessage>,
    p: Popen,
}

impl FaceTracker {
    fn new(
        p: Popen,
        control_rx: Receiver<ControlMessage>,
        cleanup: Box<dyn FnMut()>,
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
        while let Ok(cm) = self.control_rx.try_recv() {
            match cm {
                ControlMessage::Calibrate => {
                    self.p
                        .stdin
                        .as_ref()
                        .unwrap()
                        .write_all("calibrate\n".as_bytes())
                        .unwrap();
                }
                ControlMessage::Die => {
                    panic!("this should quit gracefully")
                }
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

        self.p
            .stdin
            .as_ref()
            .unwrap()
            .write_all(&[NEWLINE])
            .unwrap();

        line
    }
}

impl Iterator for FaceTracker {
    type Item = TrackingReport;

    fn next(&mut self) -> Option<TrackingReport> {
        loop {
            self.poll();
            self.handle_input();

            let line = self.read_line();

            if !line.is_empty() {
                let report: TrackingReport = match serde_json::from_str(&line) {
                    Ok(r) => r,
                    Err(e) => panic!("got bad data from tracker: {} ({})", line, e),
                };
                return Some(report);
            }
        }
    }
}

pub fn run_tracker(
    control_rx: Receiver<ControlMessage>,
    options: &Options,
) -> Result<FaceTracker, RunTrackerError> {
    let mut tracker_bin = File::create(TRACKER_BIN_PATH.as_path())?;

    #[cfg(not(debug_assertions))]
    tracker_bin.write_all(bin::TRACKER_BIN)?;

    tracker_bin.flush()?;

    let metadata = tracker_bin.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(TRACKER_BIN_PATH.as_path(), permissions)?;

    drop(tracker_bin);

    let mut args: Vec<&OsStr> = Vec::new();

    #[cfg(not(debug_assertions))]
    args.push(TRACKER_BIN_PATH.as_path().as_os_str());

    #[cfg(debug_assertions)]
    args.extend(["python", "src/py/layertuber/__init__.py"].map(OsStr::new));

    let camera_index = format!("--camera={}", options.camera_index);
    args.push(OsStr::new(&camera_index));

    if options.show_features {
        args.push(OsStr::new("--show-features"))
    }

    let p = Popen::create(
        &args,
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
                Ok(_) => info!("deleted tracker"),
                Err(e) => error!("tracker deletion failed: {}", e),
            };
        }),
    );

    Ok(tracker)
}
