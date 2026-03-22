use std::error::Error;
use std::path::Path;

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
        image_data.resize(width, height, image::imageops::FilterType::Lanczos3)
    };

    resized_image.save_with_format(image_save_path, image::ImageFormat::Png)?;

    Ok(())
}

pub async fn mk_image_file_thumb(image_save_path: &str) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_save_path)?;
    let resized = img.resize(300, 200, image::imageops::FilterType::Lanczos3);
    let input_path = Path::new(image_save_path);
    let parent = input_path.parent().unwrap_or_else(|| Path::new(""));
    let stem = input_path
        .file_stem()
        .ok_or("missing file stem")?
        .to_string_lossy();
    let ext = input_path
        .extension()
        .ok_or("missing file extension")?
        .to_string_lossy();
    let thumb_path = parent.join(format!("{}_thumb.{}", stem, ext));
    resized.save(&thumb_path)?;
    Ok(())
}
