use argparse::{ArgumentParser, Store, StoreTrue};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, sync_channel};
use std::thread;

mod puppet;
mod tracker;

#[derive(Debug, Clone)]
pub struct Options {
    path: PathBuf,

    camera_index: u8,
    show_features: bool,
}

impl Options {
    fn from_arguments() -> Self {
        let mut path_str = String::new();
        let mut camera_index: u8 = 0;
        let mut show_features = false;

        {
            let mut parser: ArgumentParser = ArgumentParser::new();
            parser.refer(&mut path_str).required().add_argument(
                "puppet",
                Store,
                concat!(
                    "The path of the OpenRaster file you want to use as a puppet. ",
                    "Alongside the .ora file, there should be a .ora.layertuber.yaml configuration."
                ),
            );

            parser.refer(&mut camera_index).add_option(
                &["-c", "--camera"],
                Store,
                concat!(
                    "The index of the camera to use. ",
                    "If your computer has only one webcam, you can leave this at its default 0."
                ),
            );

            parser.refer(&mut show_features).add_option(
                &["--show-features"],
                StoreTrue,
                "Show an additional window with your webcam feed and facial feature detection spots overlaid on it."
            );

            parser.parse_args_or_exit();
        }

        return Options {
            path: Path::new(&path_str).into(),
            camera_index,
            show_features,
        };
    }
}

fn main() {
    let options = Options::from_arguments();

    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    let tracker_options = options.clone();
    thread::spawn(move || {
        let tracker =
            tracker::run_tracker(control_rx, &tracker_options).expect("could not start tracker");
        for report in tracker {
            report_tx.send(report).unwrap()
        }
    });

    puppet::run_puppet(options.path.as_path(), report_rx, control_tx);
}
