// https://github.com/knsd/tokio-ping/releases

use futures::{Future, Stream};

pub async fn mk_lib_network_ping(addr: String) {
    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(addr).stream()));
    let future = stream.and_then(|stream| {
        stream.take(3).for_each(|mb_time| {
            match mb_time {
                Some(time) => println!("time={}", time),
                None => println!("timeout"),
            }
            Ok(())
        })
    });
    tokio::run(future.map_err(|err| {
        eprintln!("Error: {}", err)
    }))
}