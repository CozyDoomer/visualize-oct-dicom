use dicom_object::{Tag, open_file};
use dicom_object::mem::{InMemDicomObject};
use dicom_core::{PrimitiveValue, Error};
use dicom_core::header::{DataElement};
use dicom_core::value::{Value as DicomValue};
use dicom_dictionary_std::StandardDataDictionary;

use smallvec::SmallVec;
use ndarray::{ArrayBase, Array, OwnedRepr, Dim, IxDynImpl};

use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

fn dicom_element_i32(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> i32{
    dicom_element.value().primitive().unwrap().int32().unwrap()
}

fn dicom_element_u16(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> u16{
    dicom_element.value().primitive().unwrap().uint16().unwrap()
}

fn dicom_element_smallvec_f64(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> &SmallVec<[f64; 2]>{
    match dicom_element.value().primitive().unwrap() {
        PrimitiveValue::F64(v) => Some(v),
        _ => None,
    }.unwrap()
}

fn dicom_element_vec_u16(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> Result<&[u16], Error>{
    match dicom_element.value() {
        DicomValue::Primitive(PrimitiveValue::U16(v)) => Ok(v),
        _ => Err(Error::UnexpectedDataValueLength),
    }
}

fn dicom_element_vec_u8(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> Result<&[u8], Error>{
    match dicom_element.value() {
        DicomValue::Primitive(PrimitiveValue::U8(v)) => Ok(v),
        _ => Err(Error::UnexpectedDataValueLength),
    }
}


type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn load_oct(path: String) -> DynResult<(ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>, String, Vec<f64>)> {
    let oct = open_file(path)?;

    let manufacturer = oct.element_by_name("Manufacturer")?.to_str()?.to_lowercase();

    let vendor = match &manufacturer.as_str() {
        &"heidelberg engineering" => "spectralis",
        &"carl zeiss meditec" => "cirrus",
        x => x,
    };

    let pixel_data;
    pixel_data = match vendor {
        "spectralis" => Some(dicom_element_vec_u16(oct.element(Tag(0x7fe0, 0x0010))?)?.iter().map(|&e| e.to_be() as u8).collect::<Vec<u8>>()),
        "cirrus" => Some(dicom_element_vec_u8(oct.element(Tag(0x7fe0, 0x0010))?)?.to_vec()),
        x => None,
    }.expect("could not read pixel_data");

    let slice_spacing = dicom_element_smallvec_f64(oct.element_by_name("PixelSpacing")?);

    let slices = dicom_element_i32(oct.element_by_name("NumberOfFrames")?);
    let width = dicom_element_u16(oct.element_by_name("Rows")?);
    let height = dicom_element_u16(oct.element_by_name("Columns")?);
    let shape = vec![slices as usize, width as usize, height as usize];

    let pixel_volume = Array::from_shape_vec(shape, pixel_data).expect("invalid shape");

    let min_pos = [2.161817789077759, 1.304476261138916];
    let x_max = min_pos[0] + (height as f64 * slice_spacing[0]);

    let reference_pos = vec![min_pos[0], min_pos[1], x_max, min_pos[1] + slices as f64 * slice_spacing[0]];

    Ok((pixel_volume, String::from(vendor), reference_pos)) 
}
