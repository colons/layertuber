use std::env;
use std::path::Path;
use subprocess::{ExitStatus, Popen, PopenConfig};

fn build_pyinstaller() {
    if let Ok(profile) = env::var("PROFILE") {
        if profile != "release" {
            return;
        }
    }

    env::set_current_dir(Path::new("src/lib/py")).expect("could not enter python build directory");

    let mut p = Popen::create(
        &[
            "pyinstaller",
            "--onefile",
            "--paths",
            "layertuber/vendor/OpenSeeFace/",
            "--collect-data",
            "layertuber",
            "--collect-data",
            "OpenSeeFace",
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

fn main() {
    println!("cargo:rerun-if-changed=src/py/layertuber");
    build_pyinstaller();
}
