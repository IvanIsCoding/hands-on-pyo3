use pyo3::prelude::*;
use pyo3::types::PyBytes;
use numpy::{PyArray3, PyReadonlyArrayDyn};
use ndarray::{Array3, Axis};
use jxl_oxide::{JxlImage, PixelFormat};
use jxl_oxide::color::ColourSpace;

/// Decode JPEG XL image bytes and return as NumPy array (8-bit RGB/mono only)
#[pyfunction]
fn decode_jxl_bytes(py: Python, jxl_bytes: &PyBytes) -> PyResult<PyObject> {
    let bytes = jxl_bytes.as_bytes();
    
    // Decode the JPEG XL image
    let mut image = JxlImage::builder()
        .read(std::io::Cursor::new(bytes))
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read JXL image: {}", e)))?;
    
    // Render the image
    let render_result = image
        .render_frame(0)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to render JXL frame: {}", e)))?;
    
    // Get image metadata
    let image_header = image.image_header();
    let width = image_header.size.width as usize;
    let height = image_header.size.height as usize;
    let channels = match image_header.metadata.colour_encoding.colour_space {
        jxl_oxide::ColourSpace::Rgb => 3,
        jxl_oxide::ColourSpace::Grey => 1,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Only RGB and grayscale images are supported")),
    };
    
    // Only accept 8-bit data
    let data = match render_result {
        jxl_oxide::RenderResult::U8(data) => data,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Only 8-bit images are supported")),
    };
    
    // Create ndarray from the data
    let array = Array3::from_shape_vec((height, width, channels), data)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to create array: {}", e)))?;
    
    // Convert to PyArray3 and return
    Ok(PyArray3::from_array(py, &array).to_object(py))
}

/// Python module
#[pymodule]
fn jxl_decoder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_jxl_bytes, m)?)?;
    Ok(())
}