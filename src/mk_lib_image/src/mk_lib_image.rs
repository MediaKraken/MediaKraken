#![cfg_attr(debug_assertions, allow(dead_code))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde_json::json;
use stdext::function_name;

pub fn mk_image_file_resize(base_image_path: &str, image_save_path: &str, width: u32, height: u32) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let tiny = image::open(base_image_path).unwrap();
    let scaled = tiny.resize(width, height, image::imageops::FilterType::Nearest);
    let mut output = std::fs::File::create(image_save_path).unwrap();
    scaled
        .write_to(&mut output, image::ImageFormat::Png)
        .unwrap();
}
