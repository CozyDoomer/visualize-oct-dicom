use ndarray::{ArrayBase, Array, OwnedRepr, Dim, IxDynImpl};
use std::iter::FromIterator;
use image::GrayImage;

pub fn ndarray_to_images(ndarr: ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>) {
    assert!(ndarr.is_standard_layout());

    let height = ndarr.shape()[1];
    let width = ndarr.shape()[2];

    let mut ind = 0;
    for bscan in ndarr.outer_iter() {
        let bscan_vec: Vec<u8> = bscan.into_slice().unwrap().to_vec();
        GrayImage::from_raw(width as u32, height as u32, bscan_vec).unwrap().save(format!("out{0}.jpg", ind)).unwrap();
        ind += 1;
    }
}

//pub fn ndarray_to_images(mut ndarr: ArrayBase<OwnedRepr<u16>, Dim<IxDynImpl>>) {
//    assert!(ndarr.is_standard_layout());
//
//    let bscan_count = ndarr.shape()[0];
//    let height = ndarr.shape()[1];
//    let width = ndarr.shape()[2];
//
//    for ind in 0..bscan_count {
//        let bscan = ndarr.index_axis_mut(Axis(0), ind);
//        let bscan_vec: Vec<u16> = bscan.into_slice().unwrap().to_vec();
//
//        let image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_vec(width as u32, height as u32, bscan_vec).unwrap();
//        image.save(format!("out{0}.png", ind)).unwrap();
//    }
//}
