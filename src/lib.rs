mod core;

use core::decode_jxl_core;
use numpy::PyArray3;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Python binding: decode JPEG XL image bytes and return as NumPy array (8-bit RGB/mono only)
#[pyfunction]
fn decode_jxl_bytes<'py>(
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

/// Declare modules and functions. This will be imported in Python as `import jxl_demo.jxl_demo_rs`
#[pymodule]
fn jxl_demo_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_jxl_bytes, m)?)?;
    Ok(())
}
