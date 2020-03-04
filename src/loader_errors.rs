use dicom_object::{Error, Tag, open_file};
use dicom_core::PrimitiveValue;

use dicom_core::header::DataElement;
use dicom_object::mem::InMemDicomObject;
use dicom_dictionary_std::StandardDataDictionary;

use ndarray::{ArrayBase, Array, OwnedRepr, Dim, IxDynImpl};

fn dicom_element_i32(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> i32{
    dicom_element.value()
                 .primitive()
                 .expect("could not read primitive")
                 .int32()
                 .expect("could not parse primitive int32")
}

fn dicom_element_u16(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) -> u16{
    dicom_element.value()
                 .primitive()
                 .expect("could not read primitive")
                 .uint16()
                 .expect("could not parse primitive uint16")
}

fn dicom_element_smallvec_f64(dicom_element: &DataElement<InMemDicomObject<StandardDataDictionary>>) 
                           -> Result<&smallvec::SmallVec<[f64; 2]>, Error>{
    let float_array = match dicom_element.value().primitive().expect("could not read primitive") {
        PrimitiveValue::F64(x) => Ok(x),
        _ => Err(Error::InvalidFormat),
    };
    
    float_array
}

//fundus
//let pixel_spacing = dicom_element_smallvec_f64(oct.element(Tag(0x0028, 0x0030))?);
//println!("pixel_spacing: {0}", type_of(pixel_spacing));
//println!("pixel_spacing: {:#?}", pixel_spacing);

pub fn load_oct(path: &String) -> Result<(ArrayBase<OwnedRepr<u8>, Dim<IxDynImpl>>, String, Vec<f64>), Error> {
    let oct = open_file(path)?;

    let manufacturer = oct.element_by_name("Manufacturer")?.to_str()?.to_lowercase();
    let vendor = match &manufacturer.as_str() {
        &"carl zeiss meditec" => "cirrus",
        x => x,
    };

    let pixel_data = oct.element(Tag(0x7fe0, 0x0010))?.value().as_u8()?;
    let slice_spacing = dicom_element_smallvec_f64(oct.element_by_name("PixelSpacing")?)?;
    
    let slices = dicom_element_i32(oct.element_by_name("NumberOfFrames")?);
    let width = dicom_element_u16(oct.element_by_name("Rows")?);
    let height = dicom_element_u16(oct.element_by_name("Columns")?);
    let shape = vec![slices as usize, width as usize, height as usize];
    //println!("shape: {:?}", shape);

    //println!("{:#?}", Array::from_shape_vec(vec![123 as usize, 123 as usize, 123 as usize], pixel_data.to_vec()));
    let pixel_volume = Array::from_shape_vec(shape, pixel_data.to_vec()).expect("could not parse pixeldata");
    //println!("pixel_volume: {0}", type_of(&pixel_volume));
    //println!("pixel_volume: {0}", &pixel_volume.len());
    //println!("pixel_volume: {:?}", &pixel_volume.shape());

    //let image = ndarray_to_images(pixel_volume);

    // TODO: read reference_pos for spectralis (best to look if the tag exists and if it does use it)
    
    //let reference_pos = dicom_element_smallvec_f64(oct.element_by_name("OphthalmicFrameLocationSequence")?);
    //println!("{:#?}", reference_coords);

    let min_pos = [2.161817789077759, 1.304476261138916];
    let x_max = min_pos[0] + (height as f64 * slice_spacing[0]);

    let reference_pos = vec![min_pos[0], min_pos[1], x_max, min_pos[1] + slices as f64 * slice_spacing[0]];
    
    // REMEMBER [slice_count, width, height] instead of [slice_count, height, width] 
    Ok((pixel_volume, String::from(vendor), reference_pos)) 
}
