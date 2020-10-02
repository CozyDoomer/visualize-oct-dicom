use dicom::object::{Tag, open_file};
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

pub fn load_oct(path: &str) -> Result<OctData, Box<dyn std::error::Error>> {
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
            let slice_u16 = oct.element(Tag(0x7fe0, 0x0010))?.uint16_slice()?;
            // convert &[u16] to Vec<u8>
            Some(slice_u16.iter().map(|&e| e.to_be() as u8).collect())
        },
        "cirrus" => Some(oct.element(Tag(0x7fe0, 0x0010))?.to_bytes()?.into_owned()),
        _ => return Err("vendor should be spectralis or cirrus".into()),
    }.expect("could not read pixel data");
    
    //Array::from_shape_vec(oct_shape, oct_pixels).expect(&format!("invalid shape: {}", oct_path));

    let slices: usize = oct.element_by_name("NumberOfFrames")?.to_int()?;
    let width: usize = oct.element_by_name("Columns")?.to_int()?;
    let height: usize = oct.element_by_name("Rows")?.to_int()?;
    let oct_shape = [slices, width, height];
    
    let fundus = open_file(Path::new(&fundus_path))?;
    let fundus_pixels = fundus.element(Tag(0x7fe0, 0x0010))?.to_bytes()?.into_owned();

    //let slices: u16 = oct.element_by_name("NumberOfFrames")?.to_int()?;
    let width: usize = fundus.element_by_name("Columns")?.to_int()?;
    let height: usize = fundus.element_by_name("Rows")?.to_int()?;
    let fundus_shape = [width, height];

    let min_pos = [2.161817789077759, 1.304476261138916];
    let slice_spacing = oct.element_by_name("PixelSpacing")?.to_float64()?;
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
