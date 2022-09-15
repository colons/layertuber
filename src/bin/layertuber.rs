struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{} :: {}", record.level(), record.args())
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

fn main() {
    if let Ok(()) = log::set_logger(&LOGGER) {
        log::set_max_level(log::LevelFilter::Info)
    };
    layertuber::cli::run_cli();
}
