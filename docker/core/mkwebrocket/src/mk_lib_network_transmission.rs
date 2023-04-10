#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/j0rsa/transmission-rpc
// transmission-rpc = "0.3.6"

use serde_json::json;
use stdext::function_name;
use transmission_rpc::types::{
    FreeSpace, Id, Nothing, Result, RpcResponse, SessionClose, Torrent, TorrentAction,
    TorrentAddArgs, TorrentAddedOrDuplicate, TorrentGetField, Torrents,
};
use transmission_rpc::TransClient;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn mk_network_transmission_login() -> Result<transmission_rpc::TransClient> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let client = TransClient::new("mkstack_transmission".parse()?);
    Ok(client)
}

pub async fn mk_network_transmission_close(
    mut transmission_client: transmission_rpc::TransClient,
) -> Result<()> {
    let response: Result<RpcResponse<SessionClose>> = transmission_client.session_close().await;
    match response {
        Ok(_) => println!("Yay!"),
        Err(_) => panic!("Oh no!"),
    }
    println!("Rpc response is ok: {}", response?.is_ok());
    Ok(())
}

pub async fn mk_network_transmission_add_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    file_url: String,
) -> Result<bool> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let add: TorrentAddArgs = TorrentAddArgs {
        // filename: Some(
        //     "https://releases.ubuntu.com/20.04/ubuntu-20.04.2.0-desktop-amd64.iso.torrent"
        //         .to_string(),
        // ),
        filename: Some(file_url),
        ..TorrentAddArgs::default()
    };
    let res: RpcResponse<TorrentAddedOrDuplicate> = transmission_client.torrent_add(add).await?;
    Ok(res.is_ok())
}

pub async fn mk_network_transmission_list_torrents(
    mut transmission_client: transmission_rpc::TransClient,
) -> Result<()> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let res: RpcResponse<Torrents<Torrent>> = transmission_client.torrent_get(None, None).await?;
    let names: Vec<&String> = res
        .arguments
        .torrents
        .iter()
        .map(|it| it.name.as_ref().unwrap())
        .collect();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "names": names }))
            .await
            .unwrap();
    }

    let res1: RpcResponse<Torrents<Torrent>> = transmission_client
        .torrent_get(
            Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
            Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
        )
        .await?;
    let first_three: Vec<String> = res1
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{}. {}",
                &it.id.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "first_three": first_three }),
        )
        .await
        .unwrap();
    }

    let res2: RpcResponse<Torrents<Torrent>> = transmission_client
        .torrent_get(
            Some(vec![
                TorrentGetField::Id,
                TorrentGetField::HashString,
                TorrentGetField::Name,
            ]),
            Some(vec![Id::Hash(String::from(
                "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
            ))]),
        )
        .await?;
    let info: Vec<String> = res2
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{:5}. {:^45} {}",
                &it.id.as_ref().unwrap(),
                &it.hash_string.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "info": info }))
            .await
            .unwrap();
    }
    Ok(())
}

pub async fn mk_network_transmission_remove_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    torrent_id: i64,
) -> Result<bool> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let res: RpcResponse<Nothing> = transmission_client
        .torrent_remove(vec![Id::Id(torrent_id)], false)
        .await?;
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Remove result": &res.is_ok() }),
        )
        .await
        .unwrap();
    }
    Ok(res.is_ok())
}

pub async fn mk_network_transmission_start_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    torrent_id: i32,
) -> Result<bool> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let res: RpcResponse<Nothing> = transmission_client
        .torrent_action(TorrentAction::Start, vec![Id::Id(1)])
        .await?;
    Ok(res.is_ok())
}

pub async fn mk_network_transmission_stop_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    torrent_id: i32,
) -> Result<bool> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let res: RpcResponse<Nothing> = transmission_client
        .torrent_action(TorrentAction::Stop, vec![Id::Id(1)])
        .await?;
    Ok(res.is_ok())
}
