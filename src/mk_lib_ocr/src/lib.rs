pub mod surya_ocr;
pub mod tesseract_ocr;

use image::GenericImageView;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct OcrResult {
    pub text: String,
    pub confidence: f32,
}

pub async fn mk_ocr_validate_image(file_path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("file not found: {}", file_path).into());
    }
    image::open(file_path)?;
    Ok(())
}

pub async fn mk_ocr_get_image_dimensions(file_path: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let img = image::open(file_path)?;
    Ok(img.dimensions())
}
