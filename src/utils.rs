use ndarray::{ArrayBase, OwnedRepr, Dim, IxDynImpl};
use image::{DynamicImage, ImageLuma8, GrayImage, FilterType, ImageFormat};
use plotters::prelude::*;


pub fn save_pixel_volume_as_jpg(volume: ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>) {
    assert!(volume.is_standard_layout());

    let height = volume.shape()[1];
    let width = volume.shape()[2];

    for bscan in volume.outer_iter() {
        let bscan_vec: Vec<u8> = bscan.to_slice().unwrap().to_vec();
        GrayImage::from_raw(width as u32, height as u32, bscan_vec).unwrap().save("out.jpg").unwrap();
    }
}

pub fn draw_plot(bscan: Vec<u8>, shape: &Vec<u16>, slice_ind: usize) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("bscan_{0}.svg", slice_ind);

    // TODO: don't save to disk, return plotted image

    let root = SVGBackend::new(&name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Original", ("sans-serif", 30))
        .build_ranged(0.0..1.0, 0.0..1.0)?;
    chart.configure_mesh().disable_mesh().draw()?;
    //let (w, h) = chart.plotting_area().dim_in_pixel();

    let height = shape[1];
    let width = shape[2];
    let image = DynamicImage::ImageLuma8(GrayImage::from_raw(width as u32, height as u32, bscan).unwrap());
    //.resize_exact(w - w / 10, h - h / 10, FilterType::Nearest);

    let elem: BitMapElement<_> = ((0.05, 0.95), image).into();

    chart.draw_series(std::iter::once(elem))?;
    
    Ok(())
}