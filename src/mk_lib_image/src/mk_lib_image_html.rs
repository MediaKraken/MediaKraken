use std::error::Error;
use wkhtmlapp::{ImageBuilder, Source};

/// Render the supplied HTML fragment to an image file at `output_path`.
///
/// `wkhtmlapp` drives the (synchronous) `wkhtmltoimage` binary, so this
/// function performs blocking work via `tokio::task::spawn_blocking` to
/// keep the async runtime free for other tasks.
pub async fn mk_lib_image_render_html(
    html_content: String,
    output_path: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        ImageBuilder::new(Source::Html(html_content))
            .output_file(&output_path)
            .build()?
            .run()?;
        Ok(())
    })
    .await?
}
