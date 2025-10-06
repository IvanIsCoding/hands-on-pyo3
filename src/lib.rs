mod core;

use core::decode_jxl_core;
use numpy::PyArray3;
use numpy::PyUntypedArrayMethods;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Python binding: decode JPEG XL image bytes and return as NumPy array (8-bit RGB/mono only)
#[pyfunction]
fn decode_jxl_as_array<'py>(
    py: Python<'py>,
    jxl_bytes: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyArray3<u8>>> {
    // Extract bytes from PyBytes to a Rust data struct jxl-oxide can read.
    // Manipulating Python data into a format Rust accepts is common in PyO3 code.
    let bytes = jxl_bytes.as_bytes();
    let cursor = std::io::Cursor::new(bytes);

    // This method reads the image in a ndarray::Array3. Implmentation details
    // are not relevant.
    let array = decode_jxl_core(cursor).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;

    // Convert to a NumPy array and return.
    Ok(PyArray3::from_array(py, &array))
}

/// Python binding: decode JPEG XL image bytes and return as Pillow Image
#[pyfunction]
fn decode_jxl<'py>(
    py: Python<'py>,
    jxl_bytes: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyAny>> {
    // Import PIL.Image module
    let pil_image = py.import("PIL.Image")?;

    // Get the NumPy array from our existing function
    let np_array = decode_jxl_as_array(py, jxl_bytes)?;

    // Get the shape to determine if it's mono or RGB
    let shape = np_array.shape();

    // Create Pillow Image from NumPy array
    let pil_img = if shape[2] == 3 {
        // RGB case: use mode "RGB"
        pil_image.call_method1("fromarray", (np_array, "RGB"))?
    } else if shape[2] == 1 {
        // Monochrome case: squeeze the last dimension and use mode "L"
        let squeezed = np_array.call_method1("squeeze", (2,))?;
        pil_image.call_method1("fromarray", (squeezed, "L"))?
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported number of channels",
        ));
    };

    Ok(pil_img)
}

/// jxl_demo_rs contains the functions to read JPEG XL bytes into Python imaging objects.
///
/// This declares a module. The module will be imported in Python as `import jxl_demo.jxl_demo_rs`
#[pymodule]
fn jxl_demo_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_jxl_as_array, m)?)?;
    m.add_function(wrap_pyfunction!(decode_jxl, m)?)?;
    Ok(())
}
