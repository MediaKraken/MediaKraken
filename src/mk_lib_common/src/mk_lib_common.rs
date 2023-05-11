use mk_lib_logging::mk_lib_logging;
use serde_json::json;

pub async fn print_type_of_variable<T>(_: &T) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "data_type": std::any::type_name::<T>() }),
        )
        .await
        .unwrap();
    }
}
