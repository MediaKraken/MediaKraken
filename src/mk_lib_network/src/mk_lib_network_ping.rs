#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/knsd/tokio-ping/releases

use futures::{Future, Stream};
use stdext::function_name;
use serde_json::json;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn mk_lib_network_ping(addr: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(addr).stream()));
    let future = stream.and_then(|stream| {
        stream.take(3).for_each(|mb_time| {
            match mb_time {
                Some(time) => println!("time={}", time),
                None => eprintln!("timeout"),
            }
            Ok(())
        })
    });
    tokio::run(future.map_err(|err| eprintln!("Error: {}", err)))
}
