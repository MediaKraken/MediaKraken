#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/phw/rust-discid
// discid = "0.4.4"

use discid::{DiscId, Features};

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

fn print_disc_info(disc: DiscId) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "DiscId": disc.id(), "TOC": disc.toc_string(), "Submitvia": disc.submission_url() }))
                .await;
    }
}

pub async fn mk_metadata_biscid() {
    let offsets = [
        242457, 150, 44942, 61305, 72755, 96360, 130485, 147315, 164275, 190702, 205412, 220437,
    ];
    let result = DiscId::put(1, &offsets);
    match result {
        Ok(disc) => print_disc_info(disc),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
