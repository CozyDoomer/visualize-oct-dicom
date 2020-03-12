use image::{DynamicImage, GrayImage};
use image::imageops::{FilterType};
use printpdf::{PdfDocument, PdfDocumentReference, Image, Mm};
use printpdf::indices::{PdfPageIndex, PdfLayerIndex};
use rusttype::Point;
use rusttype::Scale;

const DPI: f64 = 110.0;
const PAGE_WIDTH_PIXELS: f64 = 1920.0;
const PAGE_HEIGHT_PIXELS: f64 = 1080.0;
const MM_PIXEL_CONV: f64 = 3.77952755906;
const PIXELS_BETWEEN_IMAGES: f64 = 80.0;

pub fn save_oct_volume_as_jpgs(volume: &Vec<u8>, size: &[usize]) {
    for (i, bscan) in volume.chunks_exact(size.iter().product()).enumerate() {
        let bscan_vec: Vec<u8> = bscan.to_vec();
        GrayImage::from_raw(size[0] as u32, size[1] as u32, bscan_vec)
            .unwrap()
            .save(format!("results/{0}.jpg", i))
            .unwrap();
    }
}

//pub fn draw_mask(bscan: Vec<u8>, shape: &Vec<u16>, slice_ind: usize) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn calculate_image_positions(shape: &[usize]) -> (Point<f64>, Point<f64>) {
    let im1_x = (PAGE_WIDTH_PIXELS - shape[0] as f64 * 2.0) / 2.0 - PIXELS_BETWEEN_IMAGES / 2.0;
    let im2_x = im1_x + shape[0] as f64 + PIXELS_BETWEEN_IMAGES;
    let im_y = (PAGE_HEIGHT_PIXELS - shape[1] as f64) / 2.0 - 20.0;
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
    PdfDocument::new("oct mask comparison", 
                     Mm(PAGE_WIDTH_PIXELS / MM_PIXEL_CONV), 
                     Mm(PAGE_HEIGHT_PIXELS / MM_PIXEL_CONV), 
                     "en-face projection")
}

pub fn add_pdf_page(pdf_doc: &PdfDocumentReference, slice_ind: usize) -> (PdfPageIndex, PdfLayerIndex) {
    pdf_doc.add_page(Mm(PAGE_WIDTH_PIXELS / MM_PIXEL_CONV), 
                     Mm(PAGE_HEIGHT_PIXELS / MM_PIXEL_CONV), 
                     format!("Scan: {0}", slice_ind))
}

pub fn calculate_size_from_1side(size: &[usize], max_image_size: usize) -> ([usize; 2], FilterType) {
    let result;
    let filtertype = if size[1] > max_image_size { FilterType::Nearest } else { FilterType::CatmullRom };

    if size[0] < size[1] {
        result = [((max_image_size as f32 / size[1] as f32) * size[0] as f32).round() as usize, max_image_size];
    } else if size[0] > size[1] {
        result = [max_image_size, ((max_image_size as f32 / size[0] as f32) * size[1] as f32).round() as usize];
    } else {
        result = [max_image_size, max_image_size];
    } 
    (result, filtertype)
}

pub fn add_2d_image_to_pdf(bscan: Vec<u8>, 
                           size: &[usize], 
                           requested_size: Option<&[usize]>,
                           filtertype: Option<FilterType>,
                           pdf_doc: (&PdfDocumentReference, PdfPageIndex, PdfLayerIndex),
                           _slice_ind: usize) -> Result<(), Box<dyn std::error::Error>> 
{
    let (doc, page, layer) = pdf_doc;
    let current_layer = doc.get_page(page).get_layer(layer);

    let mut image = DynamicImage::ImageLuma8(GrayImage::from_raw(size[0] as u32, size[1] as u32, bscan)
        .expect("could not parse grayscale image"));

    image = match requested_size {
        Some(size) => image.resize_exact(size[0] as u32, 
                                         size[1] as u32, 
                                         filtertype.unwrap_or(FilterType::Nearest)),
        None    => image,
    };

    let pdf_image = Image::from_dynamic_image(&image);
    let pdf_image_todo = Image::from_dynamic_image(&image);

    let (pos1, pos2) = calculate_image_positions(requested_size.unwrap_or(size));

    pdf_image.add_to_layer(current_layer.clone(), 
                           Some(Mm(pos1.x / MM_PIXEL_CONV)), 
                           Some(Mm(pos1.y / MM_PIXEL_CONV)), 
                           None, 
                           None, 
                           None, 
                           Some(DPI));

    pdf_image_todo.add_to_layer(current_layer.clone(), 
                                Some(Mm(pos2.x / MM_PIXEL_CONV)), 
                                Some(Mm(pos2.y / MM_PIXEL_CONV)), 
                                None, 
                                None, 
                                None, 
                                Some(DPI));
    Ok(())
}
