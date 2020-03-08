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
    //println!("{:?}", oct.pixel_volume);
    //utils::save_pixel_volume_as_jpg(oct.pixel_volume);

    for (i, bscan) in oct.pixel_volume.outer_iter().enumerate() {

        let bscan_vec: Vec<u8> = bscan.to_slice().unwrap().to_vec();
        
        utils::draw_plot(bscan_vec, &oct.shape, i).expect("could not plot oct volume");
    };

    Ok(())
}
