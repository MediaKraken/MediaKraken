use crate::{OcrResult, mk_ocr_validate_image};
use std::error::Error;
use std::process::Command;

pub async fn mk_ocr_surya(file_path: &str) -> Result<OcrResult, Box<dyn Error>> {
    mk_ocr_validate_image(file_path).await?;

    let file_path_owned = file_path.to_owned();
    let result = tokio::task::spawn_blocking(move || {
        let output = Command::new("surya")
            .arg("recognize")
            .arg(&file_path_owned)
            .arg("--output_dir")
            .arg("/tmp")
            .output()
            .map_err(|e| format!("failed to execute surya: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("surya failed: {}", stderr).into());
        }

        let text = String::from_utf8_lossy(&output.stdout).to_string();
        Ok::<OcrResult, Box<dyn Error>>(OcrResult {
            text,
            confidence: 0.0,
        })
    })
    .await
    .map_err(|e| format!("blocking task join error: {e}"))??;

    Ok(result)
}

pub async fn mk_ocr_surya_with_lang(
    file_path: &str,
    lang: &str,
) -> Result<OcrResult, Box<dyn Error>> {
    mk_ocr_validate_image(file_path).await?;

    let file_path_owned = file_path.to_owned();
    let lang_owned = lang.to_owned();
    let result = tokio::task::spawn_blocking(move || {
        let output = Command::new("surya")
            .arg("recognize")
            .arg(&file_path_owned)
            .arg("--languages")
            .arg(&lang_owned)
            .arg("--output_dir")
            .arg("/tmp")
            .output()
            .map_err(|e| format!("failed to execute surya: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("surya failed: {}", stderr).into());
        }

        let text = String::from_utf8_lossy(&output.stdout).to_string();
        Ok::<OcrResult, Box<dyn Error>>(OcrResult {
            text,
            confidence: 0.0,
        })
    })
    .await
    .map_err(|e| format!("blocking task join error: {e}"))??;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_ocr_surya_missing_file() {
        let result = mk_ocr_surya("/nonexistent/file.png").await;
        assert!(result.is_err());
    }
}
