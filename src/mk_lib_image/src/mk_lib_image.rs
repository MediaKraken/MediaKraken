pub async fn mk_image_file_resize(
    base_image_path: &str,
    image_save_path: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    if width == 0 || height == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "width and height must be greater than zero",
        )
        .into());
    }

    let image_data = image::open(base_image_path)?;

    let resized_image = if image_data.width() == width && image_data.height() == height {
        image_data
    } else {
        image_data.resize(width, height, image::imageops::FilterType::Nearest)
    };

    resized_image.save_with_format(image_save_path, image::ImageFormat::Png)?;

    Ok(())
}
