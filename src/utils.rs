use ndarray::{ArrayBase, Array, OwnedRepr, Dim, IxDynImpl};
use std::iter::FromIterator;
use image::GrayImage;


fn ndarray_to_images(arr: ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>) {
    assert!(arr.is_standard_layout());

    println!("type: {:?}", type_of(&arr.shape()));
    let height = arr.shape()[1];
    let width = arr.shape()[2];
    println!("shape: {:?}", arr.shape());
    println!("height: {0}, width: {1}", height, width);
    println!("type: {:?}", type_of(&arr));

    let mut ind = 0;
    for row in arr.outer_iter() {
        println!("shape: {:?}", row.shape());
        let flat_vec = Array::from_iter(row.iter())
                         .into_raw_vec()
                         .into_iter()
                         .cloned()
                         .collect();

        GrayImage::from_raw(width as u32, height as u32, flat_vec).unwrap().save(format!("out{0}.png", ind)).unwrap();
        ind += 1;
    }
}
