use ndarray::{ArrayBase, OwnedRepr, Dim, IxDynImpl};
use image::GrayImage;

pub fn volume_to_images(volume: ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>) {
    assert!(volume.is_standard_layout());

    let height = volume.shape()[1];
    let width = volume.shape()[2];

    let mut ind = 0;
    for bscan in volume.outer_iter() {
        let bscan_vec: Vec<u8> = bscan.to_slice().unwrap().to_vec();
        GrayImage::from_raw(width as u32, height as u32, bscan_vec).unwrap().save(format!("out{0}.jpg", ind)).unwrap();
        ind += 1;
    }
}
