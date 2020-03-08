use dicom_object::{Tag, open_file};
use dicom_object::mem::{InMemDicomObject};
use dicom_core::{PrimitiveValue};
use dicom_core::header::{DataElement};
use dicom_core::value::{Value as DicomValue};
use dicom_dictionary_std::StandardDataDictionary;

use ndarray::{ArrayBase, Array, OwnedRepr, Dim, IxDynImpl};

pub struct OctVolume {
    pub pixel_volume: ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>,
    pub shape: Vec<u16>,
    pub vendor: String,
    pub reference_pos: Vec<f64>,
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

pub fn load_oct(path: String) -> Result<OctVolume, Box<dyn std::error::Error>> {
    let oct = open_file(path)?;

    let manufacturer = oct.element_by_name("Manufacturer")?.to_str()?.to_lowercase();

    let vendor = match &manufacturer.as_str() {
        &"heidelberg engineering" => "spectralis",
        &"carl zeiss meditec" => "cirrus",
        x => x,
    };

    let pixel_data = match vendor {
        "spectralis" => {
            let slice_u16 = dicom_element_slice_u16(oct.element(Tag(0x7fe0, 0x0010))?)?;
            // convert &[u16] to Vec<u8>
            Some(slice_u16.iter().map(|&e| e.to_be() as u8).collect::<Vec<u8>>())
        },
        "cirrus" => Some(dicom_element_slice_u8(oct.element(Tag(0x7fe0, 0x0010))?)?.to_vec()),
        _ => return Err("vendor should be spectralis or cirrus".into()),
    }.expect("could not read pixel data");

    let slices: u16 = dicom_element_i32(oct.element_by_name("NumberOfFrames")?) as u16;
    let width = dicom_element_u16(oct.element_by_name("Rows")?);
    let height = dicom_element_u16(oct.element_by_name("Columns")?);
    let shape = vec![slices as usize, width as usize, height as usize];

    let min_pos = [2.161817789077759, 1.304476261138916];
    let slice_spacing = dicom_element_f64(oct.element_by_name("PixelSpacing")?);
    let x_max = min_pos[0] + (height as f64 * slice_spacing);

    Ok(
        OctVolume
        {
            pixel_volume: Array::from_shape_vec(shape, pixel_data).expect("invalid shape"),
            shape: vec![slices, width, height],
            vendor: String::from(vendor), 
            reference_pos: vec![min_pos[0], min_pos[1], x_max, min_pos[1] + slices as f64 * slice_spacing]
        }
    )
}
