pub async fn mk_image_file_resize(
    base_image_path: &str,
    image_save_path: &str,
    width: u32,
    height: u32,
) {
    if width == 0 || height == 0 {
        return;
    }

    let image_data = match image::open(base_image_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "mk_image_file_resize: failed to open image '{}': {}",
                base_image_path, err
            );
            return;
        }
    };

    let resized_image = if image_data.width() == width && image_data.height() == height {
        image_data
    } else {
        image_data.resize(width, height, image::imageops::FilterType::Nearest)
    };

    if let Err(err) = resized_image.save_with_format(image_save_path, image::ImageFormat::Png) {
        eprintln!(
            "mk_image_file_resize: failed to save image '{}': {}",
            image_save_path, err
        );
    }
}
