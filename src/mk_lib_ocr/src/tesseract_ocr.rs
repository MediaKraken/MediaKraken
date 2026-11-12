use crate::{OcrResult, mk_ocr_validate_image};
use std::error::Error;

pub async fn mk_ocr_tesseract(file_path: &str) -> Result<OcrResult, Box<dyn Error>> {
    mk_ocr_validate_image(file_path).await?;

    let file_path_owned = file_path.to_owned();
    let result = tokio::task::spawn_blocking(move || {
        let mut tesseract = tesseract::Tesseract::new(None, None)
            .map_err(|e| format!("tesseract init failed: {e}"))?
            .set_image(&file_path_owned)
            .map_err(|e| format!("tesseract set_image failed: {e}"))?
            .recognize()
            .map_err(|e| format!("tesseract recognize failed: {e}"))?;
        let text = tesseract
            .get_text()
            .map_err(|e| format!("tesseract get_text failed: {e}"))?;
        let confidence = tesseract.mean_text_conf() as f32;
        Ok::<OcrResult, String>(OcrResult { text, confidence })
    })
    .await
    .map_err(|e| format!("blocking task join error: {e}"))??;

    Ok(result)
}

pub async fn mk_ocr_tesseract_with_lang(
    file_path: &str,
    lang: &str,
) -> Result<OcrResult, Box<dyn Error>> {
    mk_ocr_validate_image(file_path).await?;

    let file_path_owned = file_path.to_owned();
    let lang_owned = lang.to_owned();
    let result = tokio::task::spawn_blocking(move || {
        let mut tesseract = tesseract::Tesseract::new(None, Some(&lang_owned))
            .map_err(|e| format!("tesseract init failed: {e}"))?
            .set_image(&file_path_owned)
            .map_err(|e| format!("tesseract set_image failed: {e}"))?
            .recognize()
            .map_err(|e| format!("tesseract recognize failed: {e}"))?;
        let text = tesseract
            .get_text()
            .map_err(|e| format!("tesseract get_text failed: {e}"))?;
        let confidence = tesseract.mean_text_conf() as f32;
        Ok::<OcrResult, String>(OcrResult { text, confidence })
    })
    .await
    .map_err(|e| format!("blocking task join error: {e}"))??;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_ocr_validate_image_missing_file() {
        let result = mk_ocr_validate_image("/nonexistent/file.png").await;
        assert!(result.is_err());
    }
}
