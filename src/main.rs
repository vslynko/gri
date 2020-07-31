#[macro_use]
extern crate log;
extern crate simple_logger;

use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, StructOpt)]
#[structopt(name="Grab Images", about = "Grab and dedublicate images.")]
struct Opt {
    /// List of input locations
    #[structopt(short = "i", long = "input-list")]
    input_list: PathBuf,
}

fn main() {
    simple_logger::init().expect("Cannot initialize logging");
    let opt = Opt::from_args();
    println!("{:?}", opt);
    info!("Grab Images version {}", VERSION);

    let input_txt = fs::read_to_string(opt.input_list).unwrap();
    let lines = input_txt.lines();

    for line in lines {
        println!("Line {}", line);
    }
}
