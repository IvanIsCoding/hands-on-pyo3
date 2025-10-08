use jxl_oxide::JxlImage;
use ndarray::ArrayD;

pub(crate) fn decode_jxl_core<R: std::io::Read>(reader: R) -> Result<ArrayD<u8>, String> {
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

    // Convert render to u8 buffer
    let buffer = render.image_all_channels();
    let data: Vec<u8> = buffer
        .buf()
        .iter()
        .map(|&v| (v * 255.0).round() as u8)
        .collect();

    // Calculate actual channels from buffer size
    let total_pixels = width * height;
    let actual_channels = data.len() / total_pixels;

    // Verify the calculation makes sense
    if data.len() != total_pixels * actual_channels {
        return Err(format!(
            "Buffer size mismatch: got {} bytes, expected {} ({}x{}x{})",
            data.len(),
            total_pixels * actual_channels,
            height,
            width,
            actual_channels
        ));
    }

    // Validate channels
    if ![1, 3].contains(&actual_channels) {
        return Err(format!(
            "Unsupported number of channels: {}",
            actual_channels
        ));
    }

    // Create dynamic ndarray
    let array = ArrayD::from_shape_vec(vec![height, width, actual_channels], data)
        .map_err(|e| format!("Failed to create array: {}", e))?;

    Ok(array)
}
