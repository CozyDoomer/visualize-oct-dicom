/*
generates pdfs from subfolders of oct volumes and corresponding probability mask
*/

mod loader;
mod utils;

use std::fs::File;
use std::io::BufWriter;
use quicli::prelude::*;
use structopt::StructOpt;
use std::convert::TryInto;
use std::time::Instant;

#[derive(StructOpt)]
#[structopt(name = "pdf-generation", about = "Uses oct volumes (.dcm) and corresponding mask files to generate pdf.")]
struct Cli {
    #[structopt(short = "v", long = "volume_path", 
                help = "path to the input dataset where subfolders are scan-ids containing A-Scans")]
    volume_path: String,
    #[structopt(short = "s", long = "max_image_size", 
                help = "max size of the larger image dimension. smaller dimension will be resized proportionally")]
    max_size: usize,
}

fn main() -> CliResult {
    let _now = Instant::now();
    let args = Cli::from_args();

    let oct_data = loader::load_oct(args.volume_path).expect("could not read dicom oct volume");

    let (doc, page, layer) = utils::get_pdf_document();


    utils::add_2d_image_to_pdf(oct_data.fundus_image, &oct_data.fundus_shape, None, None, (&doc, page, layer), 0)
        .expect("could not save images as pdf");
    
    let (requested_size, filtertype) = utils::calculate_size_from_1side(&oct_data.oct_shape[1..], 256);

    let bscan_pixel_len = oct_data.oct_shape[1..].iter().product();
    utils::save_oct_volume_as_jpgs(&oct_data.oct_volume, &oct_data.oct_shape[1..]);

    for (i, bscan) in oct_data.oct_volume.chunks_exact(bscan_pixel_len).enumerate() {
        let (page, layer) = utils::add_pdf_page(&doc, i);

        utils::add_2d_image_to_pdf(bscan.to_vec(), &oct_data.oct_shape[1..], Some(&requested_size), Some(filtertype), (&doc, page, layer), i)
            .expect("could not save images as pdf");
    }

    doc.save(&mut BufWriter::new(File::create(format!("results/oct_{0}.pdf", "scanID")).unwrap())).unwrap();

    println!("{}", _now.elapsed().as_nanos());
    Ok(())
}
