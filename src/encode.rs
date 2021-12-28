use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

pub fn write_image(pixels: &[u8], file_name: &str, bounds: (usize, usize)) -> Result<(), std::io::Error>{
    let f = File::create(file_name)?;
    let e = PNGEncoder::new(f);

    e.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}
