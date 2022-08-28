use std::env;
use std::path::Path;
use subprocess::{ExitStatus, Popen, PopenConfig};

fn main() {
    println!("cargo:rerun-if-changed=src/py/layertuber");
    env::set_current_dir(Path::new("src/py")).expect("could not enter python build directory");

    let mut p = Popen::create(
        &[
            "pyinstaller",
            "--onefile",
            "--paths",
            "layertuber/vendor/OpenSeeFace/",
            "--collect-data",
            "layertuber",
            "--name",
            "layertuber",
            "layertuber/__init__.py",
        ],
        PopenConfig::default(),
    )
    .expect("could not start python build");
    let retcode = p.wait().expect("could not get return code");
    if !retcode.success() {
        match retcode {
            ExitStatus::Exited(s) => panic!("pyinstall failed with return code {}", s),
            ExitStatus::Signaled(s) => panic!("pyinstall failed with signal {}", s),
            ExitStatus::Other(s) => panic!("pyinstall failed with... something? {}", s),
            ExitStatus::Undetermined => panic!("pyinstall failed for an unknown reason"),
        }
    }
}
