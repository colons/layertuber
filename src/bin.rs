pub use lib::options::Options;
pub use lib::tracker;

mod lib;

fn main() {
    lib::run_cli();
}
