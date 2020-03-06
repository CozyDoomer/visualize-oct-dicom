/*
generates pdfs from subfolders of oct volumes and corresponding probability mask
*/

mod loader;
mod utils;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    volume_path: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    let oct = loader::load_oct(args.volume_path).expect("could not read dicom oct volume");
    
    utils::ndarray_to_images(oct.0);

    Ok(())
}
