#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![allow(unused)]

use serde_json::json;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn print_type_of_variable<T>(_: &T) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "data_type": std::any::type_name::<T>() }),
        )
        .await;
    }
}
