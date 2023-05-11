// https://github.com/phw/rust-discid

use discid::{DiscId, Features};
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

async fn print_disc_info(disc: DiscId) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // #[cfg(debug_assertions)]
    // {
    //     mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "DiscId": disc.id(), "TOC": disc.toc_string(), "Submitvia": disc.submission_url() }))
    //             .await.unwrap()
    // }
}

pub async fn mk_metadata_biscid() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let offsets = [
        242457, 150, 44942, 61305, 72755, 96360, 130485, 147315, 164275, 190702, 205412, 220437,
    ];
    let result = DiscId::put(1, &offsets);
    match result {
        Ok(disc) => print_disc_info(disc).await,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
