use crate::{OcrResult, mk_ocr_validate_image};
use std::error::Error;

pub async fn mk_ocr_tesseract(file_path: &str) -> Result<OcrResult, Box<dyn Error>> {
    mk_ocr_validate_image(file_path).await?;

    let file_path_owned = file_path.to_owned();
    let result = tokio::task::spawn_blocking(move || {
        let mut tesseract = tesseract::Tesseract::new(None, Some(&file_path_owned))?;
        let text = tesseract.recognize()?;
        let confidence = tesseract.mean_confidence();
        Ok::<OcrResult, Box<dyn Error>>(OcrResult { text, confidence })
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
        let mut tesseract = tesseract::Tesseract::new(Some(&lang_owned), Some(&file_path_owned))?;
        let text = tesseract.recognize()?;
        let confidence = tesseract.mean_confidence();
        Ok::<OcrResult, Box<dyn Error>>(OcrResult { text, confidence })
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
