/*
generates pdfs from subfolders of oct volumes and corresponding probability mask
*/

mod loader;
use std::any::type_name;

use quicli::prelude::*;
use structopt::StructOpt;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


#[derive(StructOpt)]
struct Cli {
    volume_path: String,
}


fn main() -> CliResult {
    let args = Cli::from_args();
    //"/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/samples/cirrus/bscan.dcm";
    //"/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/samples/spectralis/bscan.dcm";
    
    let oct = loader::load_oct(args.volume_path)?;
    //{
    //    Ok(x) => x,
    //    Err(err) => panic!("could not parse volume with path: {}, Error: {}", args.volume_path, err)
    //};

    println!("pixeldata: {:#?}", oct);
    
    Ok(())
}
