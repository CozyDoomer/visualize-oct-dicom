use ndarray::{ArrayBase, OwnedRepr, Dim, IxDynImpl};
use image::{DynamicImage, GrayImage};
use printpdf::{PdfDocument, PdfDocumentReference, Image, Mm, Px};
use printpdf::indices::{PdfPageIndex, PdfLayerIndex};
//use plotters::prelude::*;
use rusttype::Point;

pub const DPI: f64 = 110.0;
pub const PAGE_WIDTH_PIXELS: f64 = 1920.0;
pub const PAGE_HEIGHT_PIXELS: f64 = 1080.0;
pub const MM_PIXEL_CONV: f64 = 3.77952755906;
pub const PIXELS_BETWEEN_IMAGES: f64 = 80.0;


pub fn save_pixel_volume_as_jpgs(volume: &ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>, shape: &Vec<u16>) {
    assert!(volume.is_standard_layout());

    for (i, bscan) in volume.outer_iter().enumerate() {
        let bscan_vec: Vec<u8> = bscan.as_slice().unwrap().to_vec();
        GrayImage::from_raw(shape[1] as u32, shape[2] as u32, bscan_vec)
            .unwrap()
            .save(format!("results/{0}.jpg", i))
            .unwrap();
    }
}

//pub fn draw_plot(bscan: Vec<u8>, shape: &Vec<u16>, slice_ind: usize) -> Result<(), Box<dyn std::error::Error>> {
//    let name = format!("bscan_{0}.svg", slice_ind);
//
//    let root = SVGBackend::new(&name, (1024, 768)).into_drawing_area();
//    root.fill(&WHITE)?;
//
//    let mut chart = ChartBuilder::on(&root)
//        .caption("Original", ("sans-serif", 30))
//        .build_ranged(0.0..1.0, 0.0..1.0)?;
//    chart.configure_mesh().disable_mesh().draw()?;
//    //let (w, h) = chart.plotting_area().dim_in_pixel();
//
//    let height = shape[1];
//    let width = shape[2];
//    let image = DynamicImage::ImageLuma8(GrayImage::from_raw(width as u32, height as u32, bscan).unwrap());
//    //.resize_exact(w - w / 10, h - h / 10, FilterType::Nearest);
//
//    let elem: BitMapElement<_> = ((0.05, 0.95), image).into();
//
//    chart.draw_series(std::iter::once(elem))?;
//    
//    Ok(())
//}

pub fn calculate_image_positions(shape: &Vec<u16>) -> (Point<f64>, Point<f64>) {
    let im1_x = (PAGE_WIDTH_PIXELS - shape[1] as f64 * 2.0) / 2.0 - PIXELS_BETWEEN_IMAGES / 2.0;
    let im2_x = im1_x + shape[1] as f64 + PIXELS_BETWEEN_IMAGES;
    let im_y = (PAGE_HEIGHT_PIXELS - shape[2] as f64) / 2.0 - 20.0;
    (
        Point {
            x: im1_x, 
            y: im_y
        }, 
        Point {
            x: im2_x, 
            y: im_y
        }
    )
}


pub fn get_pdf_document() -> (PdfDocumentReference, PdfPageIndex, PdfLayerIndex) {
    PdfDocument::new(
        "oct mask comparison", 
        Mm(PAGE_WIDTH_PIXELS / MM_PIXEL_CONV), 
        Mm(PAGE_HEIGHT_PIXELS / MM_PIXEL_CONV), 
        "en-face projection"
    )
}

pub fn save_bscan_as_pdf(bscan: Vec<u8>, 
                         shape: &Vec<u16>, 
                         pdf_document: &(PdfDocumentReference,PdfPageIndex, PdfLayerIndex), 
                         slice_ind: usize) -> Result<(), Box<dyn std::error::Error>> 
{
    let (doc, page, layer) = pdf_document;
    let (page, layer) = doc.add_page(Mm(PAGE_WIDTH_PIXELS / MM_PIXEL_CONV), 
                                     Mm(PAGE_HEIGHT_PIXELS / MM_PIXEL_CONV), 
                                     format!("Scan: {0}", slice_ind));

    let current_layer = doc.get_page(page).get_layer(layer);

    let image = DynamicImage::ImageLuma8(GrayImage::from_raw(shape[1] as u32, shape[2] as u32, bscan).unwrap());
    let pdf_image = Image::from_dynamic_image(&image);
    let pdf_image_TODO = Image::from_dynamic_image(&image);

    let (pos1, pos2) = calculate_image_positions(&shape);
    //println!("{:?}, {:?}", pos1, pos2);

    pdf_image.add_to_layer(current_layer.clone(), 
                           Some(Mm(pos1.x / MM_PIXEL_CONV)), 
                           Some(Mm(pos1.y / MM_PIXEL_CONV)), 
                           None, 
                           None, 
                           None, 
                           Some(DPI));

    pdf_image_TODO.add_to_layer(current_layer.clone(), 
                                Some(Mm(pos2.x / MM_PIXEL_CONV)), 
                                Some(Mm(pos2.y / MM_PIXEL_CONV)), 
                                None, 
                                None, 
                                None, 
                                Some(DPI));

    //pdf_image.add_to_layer(current_layer.clone(), Some(Mm(408.0)), Some(Mm(272.0)), None, None, None, Some(DPI));
    //pdf_image_TODO.add_to_layer(current_layer.clone(), Some(Mm(1000.0)), Some(Mm(272.0)), None, None, None, Some(DPI));

    Ok(())
}