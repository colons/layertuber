use argparse::{ArgumentParser, Store};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, sync_channel};
use std::thread;

mod puppet;
mod tracker;

#[derive(Debug)]
struct Options {
    path: PathBuf,
}

impl Options {
    fn from_arguments() -> Self {
        let mut path_str = String::new();

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

            parser.parse_args_or_exit();
        }

        return Options {
            path: Path::new(&path_str).into(),
        };
    }
}

fn main() {
    let options = Options::from_arguments();

    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    thread::spawn(move || {
        let tracker = tracker::run_tracker(control_rx).expect("could not start tracker");
        for report in tracker {
            report_tx.send(report).unwrap()
        }
    });

    puppet::run_puppet(options.path.as_path(), report_rx, control_tx);
}
