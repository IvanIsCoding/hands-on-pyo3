use jxl_oxide::JxlImage;
use ndarray::ArrayD;

pub(crate) fn decode_jxl_core<R: std::io::Read>(reader: R) -> Result<ArrayD<u8>, String> {
    // Decode the JPEG XL image
    let image = JxlImage::builder()
        .read(reader)
        .map_err(|e| format!("Failed to read JXL image: {}", e))?;

    // Get image metadata
    let image_header = image.image_header();
    let width = image_header.size.width as usize;
    let height = image_header.size.height as usize;

    // Render the first frame
    let render = image
        .render_frame(0)
        .map_err(|e| format!("Failed to render JXL frame: {}", e))?;

    // Use stream to get the image data with all channels (including alpha)
    let mut stream = render.stream();
    let channels = stream.channels() as usize;

    // Pre-allocate buffer with correct size
    let total_size = width * height * channels;
    let mut buffer = vec![0u8; total_size];

    // Write the stream data to buffer
    let written = stream.write_to_buffer(&mut buffer);
    if written != total_size {
        return Err(format!(
            "Buffer write size mismatch: wrote {} bytes, expected {}",
            written, total_size
        ));
    }

    // Validate channels (support grayscale, RGB, and RGBA)
    if ![1, 3, 4].contains(&channels) {
        return Err(format!("Unsupported number of channels: {}", channels));
    }

    // Create dynamic ndarray with shape [height, width, channels]
    let array = ArrayD::from_shape_vec(vec![height, width, channels], buffer).map_err(|e| {
        format!(
            "Failed to create array: {} (shape: [{}, {}, {}])",
            e, height, width, channels
        )
    })?;

    Ok(array)
}
