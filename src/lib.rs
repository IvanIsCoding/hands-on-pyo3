use pyo3::prelude::*;
use pyo3::types::PyBytes;
use numpy::PyArray3;
use ndarray::Array3;
use jxl_oxide::JxlImage;
use jxl_oxide::color::ColourSpace;

/// Decode JPEG XL image bytes and return as NumPy array (8-bit RGB/mono only)
#[pyfunction]
fn decode_jxl_bytes<'py>(py: Python<'py>, jxl_bytes: &Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyArray3<u8>>> {
    // Extract bytes from PyBytes
    let bytes = jxl_bytes.as_bytes();
    
    // Decode the JPEG XL image
    let mut image = JxlImage::builder()
        .read(std::io::Cursor::new(bytes))
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read JXL image: {}", e)))?;
    
    // Render the image
    let render = image
        .render_frame(0)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to render JXL frame: {}", e)))?;
    
    // Get image metadata
    let image_header = image.image_header();
    let width = image_header.size.width as usize;
    let height = image_header.size.height as usize;
    let channels = match image_header.metadata.colour_encoding.colour_space() {
        ColourSpace::Rgb => 3,
        ColourSpace::Grey => 1,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Only RGB and grayscale images are supported")),
    };
    
    // Convert render to u8 buffer
    let buffer = render.image_all_channels();
    let data: Vec<u8> = buffer.buf().iter().map(|&v| (v * 255.0).round() as u8).collect();
    
    // Create ndarray from the data
    let array = Array3::from_shape_vec((height, width, channels), data)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to create array: {}", e)))?;
    
    // Convert to PyArray3 and return
    Ok(PyArray3::from_array(py, &array))
}

/// Python module
#[pymodule]
fn jxl_decoder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_jxl_bytes, m)?)?;
    Ok(())
}