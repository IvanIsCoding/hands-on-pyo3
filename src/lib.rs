mod core;

use core::decode_jxl_core;
use numpy::PyArrayDyn;
use numpy::PyUntypedArrayMethods;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Python binding: decode JPEG XL image bytes and return as NumPy array (8-bit, 1/3/4 channels)
#[pyfunction]
fn decode_jxl_as_array<'py>(
    py: Python<'py>,
    jxl_bytes: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyArrayDyn<u8>>> {
    // Extract bytes from PyBytes to a Rust data struct jxl-oxide can read.
    // Manipulating Python data into a format Rust accepts is common in PyO3 code.
    let bytes = jxl_bytes.as_bytes();
    let cursor = std::io::Cursor::new(bytes);

    // This method reads the image in a ndarray::ArrayD. Implementation details
    // are not relevant.
    let array = decode_jxl_core(cursor).map_err(|e| PyValueError::new_err(e))?;

    // Convert to a NumPy array and return.
    Ok(PyArrayDyn::from_array(py, &array))
}

/// Python binding: decode JPEG XL image bytes and return as Pillow Image
#[pyfunction]
fn decode_jxl<'py>(
    py: Python<'py>,
    jxl_bytes: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyAny>> {
    // Import PIL.Image module
    let pil_image = py.import("PIL.Image")?;
    let fromarray_fn = pil_image.getattr("fromarray")?;

    // Get the NumPy array from our existing function
    let np_array = decode_jxl_as_array(py, jxl_bytes)?;

    // Get the shape to determine if it's mono or RGB
    let shape = np_array.shape();

    // Create Pillow Image from NumPy array
    let pil_img = if shape[2] == 3 || shape[2] == 4 {
        // a.k.a. RGB case
        fromarray_fn.call1((np_array,))?
    } else if shape[2] == 1 {
        // a.k.a. monochrome case
        // Some data manipulation first, then fromarray call
        let squeezed = np_array.call_method1("squeeze", (2,))?;
        fromarray_fn.call1((squeezed, "L"))?
    } else {
        // Unsupported number of channels
        return Err(PyValueError::new_err(format!(
            "Unsupported number of channels: {}. Expected 1 (grayscale), 3 (RGB), or 4 (RGBA)",
            shape[2]
        )));
    };

    Ok(pil_img)
}

/// jxl_demo contains the functions to read JPEG XL bytes into Python imaging objects.
///
/// This declares a module. The module will be imported in Python as `import jxl_demo`
#[pymodule]
fn jxl_demo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_jxl_as_array, m)?)?;
    m.add_function(wrap_pyfunction!(decode_jxl, m)?)?;
    Ok(())
}
