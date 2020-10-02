//#![feature(test)]

//extern crate test;

// current manual benchmarks:
//
// nightly debug
// --------
// 8.079842666 cirrus
// 9.261622522 spectralis
//
//
// default debug
// --------
// 8.217648467,  8.186279273  8.112488688 cirrus
// 9.835061066, 10.104345107, 9.771452252 spectralis
//
// nightly release
// --------
// 6.346898043 6.312235771 6.742419365 cirrus
// 1.744876646 1.981386831 1.933800719 spectralis

pub mod utils;
pub mod loader;

#[cfg(test)]
mod test_utils {
    use super::*;
    use rusttype::Point;
    //use test::Bencher;

    #[test]
    fn test_calculate_image_positions() {
        assert_eq!((Point{x: 408.0, y: 272.0}, Point{x: 1000.0, y: 272.0}), utils::calculate_image_positions(&[512, 496]));
        assert_eq!((Point{x: 408.0, y:   8.0}, Point{x: 1000.0, y:   8.0}), utils::calculate_image_positions(&[512, 1024]));
        assert_eq!((Point{x: 720.0, y:   8.0}, Point{x: 1000.0, y:   8.0}), utils::calculate_image_positions(&[200, 1024]));
    }

    //#[bench]
    //fn bench_calculate_image_positions(b: &mut Bencher) {
    //    b.iter(|| utils::calculate_image_positions([512, 496]));
    //    b.iter(|| utils::calculate_image_positions([512, 1024]));
    //    b.iter(|| utils::calculate_image_positions([200, 1024]));
    //}
}

//#[cfg(test)]
//mod test_loader {
//    use super::*;
//    use rand::{thread_rng, Rng};
//    use ndarray::{Array};
//    //use test_loader::Bencher;
//
//    fn mock_oct_data_cirrus(bscan_shape: &[usize; 3], fundus_shape: &[usize; 2]) -> OctData {
//        loader::OctData {
//            oct_volume: Array::from_shape_vec(
//                bscan_shape, 
//                vec!(thread_rng().gen_range(0, bscan_shape[0]*bscan_shape[1]*bscan_shape[2]))
//            ).unwrap(),
//            vendor: String::from("spectralis"),
//            reference_pos: [2.161817789077759, 1.304476261138916, 5.566997789077758, 2.8074522611389163],
//            fundus_image: thread_rng().gen_range(0, fundus_shape[0]*fundus_shape[1]),
//            fundus_shape: *fundus_shape
//        }
//    }
//
//    #[test]
//    fn test_load_oct() {
//        assert_eq!(mock_oct_data_cirrus(&[512,  496,  49], &[512, 512]), loader::load_oct(String::from("samples/cirrus")));
//        assert_eq!(mock_oct_data_cirrus(&[512, 1024, 128], &[512, 512]), loader::load_oct(String::from("samples/spectralis")));
//    }
//
//    //#[bench]
//    //fn bench_load_oct(b: &mut Bencher) {
//    //    b.iter(|| loader::load_oct(String::from("samples/cirrus")))
//    //    b.iter(|| loader::load_oct(String::from("samples/spectralis")))
//    //}
//}
