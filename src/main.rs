#[macro_use]
extern crate log;
extern crate globwalk;
extern crate simple_logger;

use std::error::Error;
use std::fs;

use std::path::PathBuf;
use structopt::StructOpt;

const FILE_TYPES_GLOB: &'static str = "*.{jpg,cr2,tiff}";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn unwind_glob_sources(sources: Vec<&str>) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for source_dir in &sources {
        info!("Source: {}", source_dir);
        let walker = globwalk::GlobWalkerBuilder::new(source_dir, FILE_TYPES_GLOB)
            .case_insensitive(true)
            .build()
            .unwrap()
            .into_iter()
            .filter_map(Result::ok);
        for img in walker {
            result.push(img.into_path());
        }
    }

    result
}

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
    debug!("{:?}", opt);
    info!("Grab Images version {}", VERSION);

    let input_txt = fs::read_to_string(opt.input_list).unwrap();
    let lines: Vec<&str> = input_txt.lines().collect();

    let all_sources = unwind_glob_sources(lines);

    for source in all_sources {
        info!("Image: {:?}", source);
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn find_jpegs() {
        simple_logger::init().expect("Cannot initialize logging");

        let test_paths = ["tests/test_root/folder1", "tests/test_root/folder2"];
        let paths: Vec<PathBuf> = test_paths
            .iter()
            .map(|tp| Path::new(tp).canonicalize().expect("wrong base path"))
            .collect();
        let sources = paths.iter().map(|pp| pp.to_str().unwrap()).collect();

        let images = unwind_glob_sources(sources);

        assert_eq!(images.len(), 4);
    }
}
