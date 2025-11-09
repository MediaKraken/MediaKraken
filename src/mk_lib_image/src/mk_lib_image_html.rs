use wkhtmlapp::{ImageBuilder, Source};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html_content = "<h1>Hello, World!</h1><p>This is some HTML.</p>";
    let output_path = "output.png";
    ImageBuilder::new(Source::Html(html_content.to_string()))
        .output_file(output_path)
        .build()?
        .run()?;
    println!("HTML rendered to image: {}", output_path);
    Ok(())
}