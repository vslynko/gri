#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init().expect("Cannot initialize logging");
    info!("Grab images v");
    println!("Hello, world!");
}
