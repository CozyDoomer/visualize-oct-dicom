/*
generates pdfs from subfolders of oct volumes and corresponding probability mask
*/

mod loader;
mod utils;

use std::fs::File;
use std::io::BufWriter;
use quicli::prelude::*;
use structopt::StructOpt;

use std::time::Instant;
use std::convert::TryInto;

#[derive(StructOpt)]
struct Cli {
    volume_path: String,
}

fn fix_to_usize2(shape: &[usize]) -> &[usize; 2] {
    shape.try_into().expect("slice with incorrect length")
}

fn main() -> CliResult {
    let now = Instant::now();
    let args = Cli::from_args();

    let oct_data = loader::load_oct(args.volume_path).expect("could not read dicom oct volume");

    let oct_shape = fix_to_usize2(&oct_data.oct_volume.shape().split_first().unwrap().1);
    let fundus_shape = fix_to_usize2(&oct_data.fundus_shape);
    //println!("{:?}", oct_data.oct_volume);
    let (doc, page, layer) = utils::get_pdf_document();
    //utils::save_oct_volume_as_jpgs(&oct_data.oct_volume, &oct.shape);

    utils::add_2d_image_to_pdf(oct_data.fundus_image,
                               fundus_shape,
                               (&doc,
                               page,
                               layer),
                               0).expect("could not save images as pdf");

    for (i, bscan) in oct_data.oct_volume.outer_iter().enumerate() {

        let bscan_vec: Vec<u8> = bscan.to_slice().unwrap().to_vec();

        //utils::draw_plot(bscan_vec, &oct_data.oct_volume.shape, i).expect("could not plot oct volume to image");

        let (page, layer) = utils::add_pdf_page(&doc, i);

        utils::add_2d_image_to_pdf(bscan_vec, 
                                   oct_shape,
                                   (&doc,
                                   page,
                                   layer),
                                   i).expect("could not save images as pdf");
    }

    doc.save(&mut BufWriter::new(File::create(format!("results/oct_{0}.pdf", "scanID")).unwrap())).unwrap();
    Ok(())
}
