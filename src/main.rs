#[macro_use]
extern crate log;
extern crate globwalk;
extern crate simple_logger;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, StructOpt)]
#[structopt(name = "Grab Images", about = "Grab and dedublicate images.")]
struct Opt {
    /// List of input locations
    #[structopt(short = "i", long = "input-list")]
    input_list: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init().expect("Cannot initialize logging");
    let opt = Opt::from_args();
    println!("{:?}", opt);
    info!("Grab Images version {}", VERSION);

    let input_txt = fs::read_to_string(opt.input_list).unwrap();
    let lines: Vec<&str> = input_txt.lines().collect();

    for line in &lines {
        println!("Input line {}", line);
    }

    let walker = globwalk::GlobWalkerBuilder::from_patterns("C:\\", &lines)
    // let walker = globwalk::GlobWalkerBuilder::from_patterns("C:\\", &["*.txt"])
        .build()?
        .into_iter()
        .filter_map(Result::ok);

    for img in walker {
        println!("Line {:?}", img.path());
    }

    return Ok(());
}
