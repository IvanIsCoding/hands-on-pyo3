# Extending Python with Rust: a hands-on introduction to PyO3

This is an example project using [PyO3](https://github.com/PyO3/pyo3). The goal
of this project is to provide a practical 

## Usage

To install from source, firstly ensure you have a Rust compiler available.

Then, simply run:

```bash
pip install .
```

Once that is done, the extension module should be available in Python:

```python
import jxl_demo
```

The `jxl_demo` module has two functions: `decode_jxl` and `decode_jxl_as_array`.

## Purpose

This example extension lets users load [JPEG XL](https://en.wikipedia.org/wiki/JPEG_XL)
images into Python as NumPy arrays and [Pillow Images](https://pillow.readthedocs.io/en/stable/reference/Image.html).

We leverage [jxl-oxide](https://docs.rs/jxl-oxide/0.12.4/jxl_oxide/) to read the JPEG XL images. This demo code wraps the Rust crate and makes it available for Python users. 