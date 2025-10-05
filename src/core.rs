use pyo3::prelude::*;
use pyo3::types::PyBytes;
use numpy::PyArray3;
use ndarray::Array3;
use jxl_oxide::JxlImage;
use jxl_oxide::color::ColourSpace;

/// Core business logic: decode JPEG XL image from any readable source
fn decode_jxl_core<R: std::io::Read>(reader: R) -> Result<Array3<u8>, String> {
    // Decode the JPEG XL image
    let image = JxlImage::builder()
        .read(reader)
        .map_err(|e| format!("Failed to read JXL image: {}", e))?;
    
    // Render the image
    let render = image
        .render_frame(0)
        .map_err(|e| format!("Failed to render JXL frame: {}", e))?;
    
    // Get image metadata
    let image_header = image.image_header();
    let width = image_header.size.width as usize;
    let height = image_header.size.height as usize;
    let channels = match image_header.metadata.colour_encoding.colour_space() {
        ColourSpace::Rgb => 3,
        ColourSpace::Grey => 1,
        _ => return Err("Only RGB and grayscale images are supported".to_string()),
    };
    
    // Convert render to u8 buffer
    let buffer = render.image_all_channels();
    let data: Vec<u8> = buffer.buf().iter().map(|&v| (v * 255.0).round() as u8).collect();
    
    // Create ndarray from the data
    let array = Array3::from_shape_vec((height, width, channels), data)
        .map_err(|e| format!("Failed to create array: {}", e))?;
    
    Ok(array)
}