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
    // Extract bytes from PyBytes
    let bytes = jxl_bytes.as_bytes();

    // Create cursor and call core logic
    let cursor = std::io::Cursor::new(bytes);
    let array = decode_jxl_core(cursor).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;

    // Convert to PyArray3 and return
    Ok(PyArray3::from_array(py, &array))
}

/// Python module
#[pymodule]
fn jxl_decoder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_jxl_bytes, m)?)?;
    Ok(())
}
