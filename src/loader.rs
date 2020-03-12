use dicom_object::{Tag, open_file};
use dicom_object::mem::{InMemDicomObject};
use dicom_core::{PrimitiveValue};
use dicom_core::header::{DataElement};
use dicom_core::value::{Value as DicomValue};
use dicom_dictionary_std::StandardDataDictionary;
use std::path::Path;

#[derive(Debug)]
pub struct OctData {
    pub oct_volume: Vec<u8>, 
    pub oct_shape: [usize; 3],
    pub vendor: String,
    pub reference_pos: [f64; 4],
    pub fundus_image: Vec<u8>,
    pub fundus_shape: [usize; 2],
}

fn dicom_element_i32(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> i32 {
    dicom_element.value().primitive().unwrap().int32().unwrap()
}

fn dicom_element_u16(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> u16 {
    dicom_element.value().primitive().unwrap().uint16().unwrap()
}

fn dicom_element_f64(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> f64 {
    dicom_element.value().primitive().unwrap().float64().unwrap()
}

fn dicom_element_slice_u16(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> Result<&[u16], &str> {
    match dicom_element.value() {
        DicomValue::Primitive(PrimitiveValue::U16(v)) => Ok(v),
        _ => Err("could not match dicom_element to &[u16]"),
    }
}

fn dicom_element_slice_u8(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> Result<&[u8], &str> {
    match dicom_element.value() {
        DicomValue::Primitive(PrimitiveValue::U8(v)) => Ok(v),
        _ => Err("could not match dicom_element to &[u8]"),
    }
}

pub fn load_oct(path: String) -> Result<OctData, Box<dyn std::error::Error>> {
    let oct_path = format!("{}/{}", path.to_string(), "/bscan.dcm");
    let fundus_path = format!("{}/{}", path.to_string(), "/fundus.dcm");
    
    let oct = open_file(Path::new(&oct_path))?;

    let manufacturer = oct.element_by_name("Manufacturer")?.to_str()?.to_lowercase();

    let vendor = match &manufacturer.as_str() {
        &"heidelberg engineering" => "spectralis",
        &"carl zeiss meditec" => "cirrus",
        x => x,
    };

    let oct_pixels = match vendor {
        "spectralis" => {
            let slice_u16 = dicom_element_slice_u16(oct.element(Tag(0x7fe0, 0x0010))?)?;
            // convert &[u16] to Vec<u8>
            Some(slice_u16.iter().map(|&e| e.to_be() as u8).collect::<Vec<u8>>())
        },
        "cirrus" => Some(dicom_element_slice_u8(oct.element(Tag(0x7fe0, 0x0010))?)?.to_vec()),
        _ => return Err("vendor should be spectralis or cirrus".into()),
    }.expect("could not read pixel data");
    
    //Array::from_shape_vec(oct_shape, oct_pixels).expect(&format!("invalid shape: {}", oct_path));

    let slices: u16 = dicom_element_i32(oct.element_by_name("NumberOfFrames")?) as u16;
    let width = dicom_element_u16(oct.element_by_name("Columns")?);
    let height = dicom_element_u16(oct.element_by_name("Rows")?);
    let oct_shape = [slices as usize, width as usize, height as usize];
    
    let fundus = open_file(Path::new(&fundus_path))?;
    let fundus_pixels = dicom_element_slice_u8(fundus.element(Tag(0x7fe0, 0x0010))?)?.to_vec();

    //let slices: u16 = dicom_element_i32(oct.element_by_name("NumberOfFrames")?) as u16;
    let width = dicom_element_u16(fundus.element_by_name("Columns")?);
    let height = dicom_element_u16(fundus.element_by_name("Rows")?);
    let fundus_shape = [width as usize, height as usize];

    let min_pos = [2.161817789077759, 1.304476261138916];
    let slice_spacing = dicom_element_f64(oct.element_by_name("PixelSpacing")?);
    let x_max = min_pos[0] + (height as f64 * slice_spacing);
    let ref_pos = [min_pos[0], min_pos[1], x_max, min_pos[1] + slices as f64 * slice_spacing];
    
    Ok(
        OctData
        {
            oct_volume: oct_pixels,
            oct_shape: oct_shape,
            vendor: String::from(vendor), 
            reference_pos: ref_pos,
            fundus_image: fundus_pixels,
            fundus_shape: fundus_shape,
        }
    )
}
