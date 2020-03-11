#![feature(test)]

extern crate test;

mod utils;
mod loader;

use rusttype::Point;

#[cfg(test)]
mod test_utils {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_calculate_image_positions() {
        assert_eq!((Point{x: 408.0, y: 272.0}, Point{x: 1000.0, y: 272.0}), utils::calculate_image_positions([512, 496]));
        assert_eq!((Point{x: 408.0, y:   8.0}, Point{x: 1000.0, y:   8.0}), utils::calculate_image_positions([512, 1024]));
        assert_eq!((Point{x: 720.0, y:   8.0}, Point{x: 1000.0, y:   8.0}), utils::calculate_image_positions([200, 1024]));
    }

    #[bench]
    fn bench_calculate_image_positions(b: &mut Bencher) {
        b.iter(|| utils::calculate_image_positions([512, 496]));
        b.iter(|| utils::calculate_image_positions([512, 1024]));
        b.iter(|| utils::calculate_image_positions([200, 1024]));
    }
}

//#[cfg(test)]
//mod test_loader {
//    use super::*;
//    use test_loader::Bencher;
//
//    fn mock_oct_data_cirrus(oct_shape: &[usize], fundus_shape: &[usize]) -> OctData {
//        OctData {
//            oct_volume: Array::from_shape_vec(oct_shape, rng.gen_range(0, oct_shape[0]*oct_shape[1])).unwrap(),
//            vendor: String::from("spectralis"),
//            reference_pos: vec![2.161817789077759, 1.304476261138916, 5.566997789077758, 2.8074522611389163],
//            fundus_image: rng.gen_range(0, fundus_shape[0]*fundus_shape[1]),
//            fundus_shape: fundus_shape
//        }
//    }
//
//    #[test]
//    fn test_load_oct() {
//        assert_eq!(mock_oct_data_cirrus(&[512, 496]), loader::load_oct(String::from("samples/cirrus")));
//        assert_eq!(mock_oct_data_cirrus(&[512, 496]), loader::load_oct(String::from("samples/spectralis")));
//    }
//
//    #[bench]
//    fn bench_load_oct(b: &mut Bencher) {
//        b.iter(|| loader::load_oct(String::from("samples/cirrus")));
//        b.iter(|| loader::load_oct(String::from("samples/spectralis")));
//    }
//}