#[macro_use]
extern crate log;
extern crate simple_logger;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    simple_logger::init().expect("Cannot initialize logging");
    info!("Grab Images version {}", VERSION);
}
